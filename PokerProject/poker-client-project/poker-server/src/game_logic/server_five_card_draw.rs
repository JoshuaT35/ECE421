/// contains game logic progression for five card draw

use std::{
    collections::HashMap,
    net::TcpStream,
    thread,
};
use rayon::prelude::*;

use crate::{
    server_communication,
    game_logic::game_utils::*,
};

use poker_model::{
    game_types::{
        dealer_game::DealerGame,
        five_card_draw::FiveCardDrawDealer,
    },
    poker_players::{
        poker_player_base::{PokerPlayer, PlayerStatus},
        poker_player_five_card_draw::PokerPlayerFiveCardDraw,
    },
    poker_deck::{
        Card,
        discard_cards,
        create_card_message,
    },
    hand_evaluator::HandEvaluator,
};

// function that begins a five card draw game
pub fn host_five_card_draw(
    players: &mut Vec<(u8, PokerPlayerFiveCardDraw, TcpStream)>,
) -> String {
    // create and setup the dealer
    let mut dealer: FiveCardDrawDealer = FiveCardDrawDealer::new();
    dealer.setup();

    // variable that determines if we end the game or not
    let mut continue_playing: bool = true;

    // stores current winners
    let mut winner_names: Vec<String> = Vec::new();

    // players end when only 1 person has chips
    while continue_playing {
        // winner(s) of the current round, and their hand(s)
        winner_names = Vec::new();
        let mut winner_hands: Vec<Card> = Vec::new();

        // inform players of their situation
        player_status_report(&mut dealer, players);

        // deduct ante from each active player
        players_pay_ante(&mut dealer, players);

        // inform players of the number of chips they have
        player_inform_chips(players);

        // --- variables to reset every round ---
        // number of players that will participate in the next round
        let mut num_players_in_betting_round: usize = num_active_players(&players);
        let mut all_in_player: bool = false;

        // conduct two phases for each game
        for round_num in 0..2 {
            // --- deal cards phase ---
            deal_cards_phase(&mut dealer, players);

            // --- betting phase ---
            if num_players_in_betting_round > 1 {
                betting_phase(&mut dealer, players, &mut num_players_in_betting_round);
            }

            for (_, player, socket) in players.iter_mut() {
                if player.get_status() == PlayerStatus::AllIn {
                    all_in_player = true;
                }
            }

            // inform players of the number of chips they have
            player_inform_chips(players);

            // if only 1 player left in round, end early
            if num_players_in_betting_round < 2 && !all_in_player {
                break;
            }

            // --- discard phase ---
            // players discard on the first round only
            if round_num == 0 {
                discard_phase(players);
            }
        }

        // --- evaluate the winner for the current hand (only if 2+ people didn't fold) --- 
        if num_players_in_betting_round > 1 || all_in_player {
            (winner_names, winner_hands) = evaluate_winner(players);
        } else {
            (winner_names, winner_hands) = all_fold(players);
        }

        // distribute Pot evenly
        distribute_pot(&mut dealer, players, &winner_names);

        // if only 1 player has chips, stop playing.
        // else, reset deck and hands for next round
        continue_playing = go_to_next_round_and_set_environment(&mut dealer, players);
    }

    // print game ended message
    inform_players_game_ended(players);

    // return winner
    return winner_names[0].clone();
}

// conduct deal cards phase in a round
fn deal_cards_phase(
    dealer: &mut FiveCardDrawDealer,
    players: &mut Vec<(u8, PokerPlayerFiveCardDraw, TcpStream)>,
) {
    for (_, player, socket) in players.iter_mut() {
        // if player is eliminated, skip
        if player.get_status() == PlayerStatus::Eliminated {
            continue;
        }

        // dealer deals cards to each player until they have 5
        player.receive_cards(dealer.deal_cards(5-player.get_num_cards()));

        // send players their hands
        send_hand(player, socket);
    }
}

// conduct discard phase in a round
fn discard_phase(players: &mut Vec<(u8, PokerPlayerFiveCardDraw, TcpStream)>) {
    // NOTE: use rayon to speed things up via par_iter_mut?
    for (_, player, socket) in players.iter_mut() {
        // if player has folded or is eliminated, skip them
        if player.get_status() == PlayerStatus::Folded
        || player.get_status() == PlayerStatus::Eliminated {
            continue;
        }

        // let players discard their cards in order
        player_discard_cards(player, socket);
    }
}

// player can discard cards
fn player_discard_cards(
    player: &mut PokerPlayerFiveCardDraw,
    socket: &mut TcpStream,
) {
    // messages to write
    let discard_message: &str = "\nWhat cards would you like to discard? Type the indices of the cards you wish to discard (1-5), or 0 to skip discard.\n";
    let invalid_message: &str = "Invalid input. Type the indices of the cards you wish to discard (1-5), or 0 to skip discard: \n";

    // write message informing them of what to do
    server_communication::write_message(socket, discard_message);

    let response = loop {
        // wait and get a response
        let response = server_communication::read_message_wait(socket);
        
        // split the response into words (space-separated) and trim each word
        let parts: Vec<&str> = response.trim().split_whitespace().collect();
        
        // check if the response is a valid list of space-separated numbers from 1 to 5, or a single 0
        if parts == ["0"] {
            // If the response is "0", break out of the loop (no discard)
            break response.trim().to_string();
        }
        else if parts.iter().all(|&s| s.parse::<usize>().is_ok()) {
            // Check if all parts are valid numbers (strings that can be parsed as usize)
            let indices: Vec<usize> = parts.iter()
                .filter_map(|&s| s.parse::<usize>().ok())
                .collect();
            
            // Check that all indices are in the range 1 to 5
            if indices.iter().all(|&index| index >= 1 && index <= 5) {
                break response.trim().to_string();
            }
        }

        // invalid response (i.e contains non-numeric input, input numbers are out-of-range, etc)
        server_communication::write_message(socket, invalid_message);
    };
    
    // after the loop, the `response` variable contains a valid response (either "0" or space-separated numbers)
    if response.as_str() != "0" {
        // remove those positions from the player hand
        // function from poker_deck.rs
        discard_cards(&mut player.hand, &response);
    };
}










    // store thread handles for each player in the game
    // let mut game_player_thread_handles = Vec::new();

    // mutex-condvar pair - for strict player ordering
    // barrier - ensures players meet up at a specific point in the game
    // let thread_sync_pair: Arc<(Mutex<u8>, Condvar)> = Arc::new((Mutex::new(0), Condvar::new()));
    // let barrier: Arc<Barrier> = Arc::new(Barrier::new(players.len()));



    // do the below until only 1 player is left standing
    // - deal cards
    // NOTE: uses Rayon, order that dealer deals in does not matter
    // players.par_iter_mut().for_each(|(turn_id, player, socket)| {
    //     // clone variables
    //     let mut dealer_ptr_clone = Arc::clone(&dealer_ptr);
    //     let mut socket_clone = socket.try_clone().unwrap();

    //     dealing_phase(&mut dealer_ptr_clone, player, &mut socket_clone);
    // });










    // for (turn_id, mut player, socket) in players.clone().into_iter() {
    //     println!("hi!");
    // }
    

        // host a betting phase
        // for each player
        // for (turn_id, mut player, socket) in players.into_iter() {

        //     // clone variables
        //     let mut dealer_ptr_clone = Arc::clone(&dealer_ptr);
        //     let mut socket_clone = socket.try_clone().unwrap();
        //     let lock_clone: Arc<(Mutex<u8>, Condvar)> = Arc::clone(&thread_sync_pair);
        //     let barrier_clone: Arc<Barrier> = Arc::clone(&barrier);
    
        //     // spawn a thread for each player
        //     let thread_player = thread::spawn(move || {
    
        //         // thread takes ownership
        //         let mut player_in_thread = player;
    
        //         // TODO: set condition for when the rounds end
        //         for _ in 0..2 {
        //             five_card_draw_round(
        //                 &mut dealer_ptr_clone,
        //                 &mut player_in_thread,
        //                 &mut socket_clone,
        //                 &lock_clone,
        //                 &barrier_clone,
        //                 turn_id,
        //             );
        //         }
    
        //         // collect all the hands
        //     });
    
        //     // store thread handles
        //     game_player_thread_handles.push(thread_player);
        // }
    
        // // have this function wait for the thread
        // for thread in game_player_thread_handles.drain(..) {
        //     thread.join().unwrap();
        // }
        // // clear the thread handle
        // game_player_thread_handles.clear();

        // host a dealing phase

        // if there are still players with chips, repeat





// fn betting_phase(
//     dealer_ptr: &mut Arc<Mutex<FiveCardDrawDealer>>,
//     player: &mut PokerPlayerFiveCardDraw,
//     socket: &mut TcpStream,
//     lock_pair: &Arc<(Mutex<u8>, Condvar)>,
//     barrier: &Arc<Barrier>,
//     turn_id: u8,
// ) {
//     // set up lock pair
//     let (mutex, cvar) = lock_pair.as_ref();

//     // reset turn to 0
//     {
//         let mut turn = mutex.lock().unwrap();
//         *turn = 0;
//     }

//     // wait for players
//     barrier.wait();

//     // --- betting phase ---
//     {
//         let mut turn = mutex.lock().unwrap();

//         // if the player's turn id does not match mutex num, wait
//         while *turn != turn_id {
//             turn = cvar.wait(turn).unwrap();
//         }
//         // player with matching mutex id proceeds

//         // get the action for each player
//         {
//             let mut dealer = dealer_ptr.lock().unwrap();
//             conduct_action(&mut *dealer, player, socket);
//         }

//         // increase turn id by 1 for the next player
//         *turn += 1;
//         // notify everone still asleep/waiting so next player proceeds
//         cvar.notify_all();
//     }
// }

// a round in five card draw bewteen the dealer and a player
// fn five_card_draw_round(
//     dealer_ptr: &mut Arc<Mutex<FiveCardDrawDealer>>,
//     player: &mut PokerPlayer,
//     socket: &mut TcpStream,
//     lock_pair: &Arc<(Mutex<u8>, Condvar)>,
//     barrier: &Arc<Barrier>,
//     turn_id: u8,
// ) {
//     // set up lock pair
//     let (mutex, cvar) = lock_pair.as_ref();

//     // reset turn to 0
//     {
//         let mut turn = mutex.lock().unwrap();
//         *turn = 0;
//     }

//     // wait for players
//     barrier.wait();

//     // --- dealer deals missing cards to players ---
//     {
//         let mut dealer = dealer_ptr.lock().unwrap();
//         deal_cards_to_player(&mut *dealer, player);
//     }

//     // --- send players their hands ---
//     send_hand(&player, socket);

//     // players wait until dealer has dealt all cards
//     barrier.wait();

//     // --- betting phase ---
//     {
//         let mut turn = mutex.lock().unwrap();

//         // if the player's turn id does not match mutex num, wait
//         while *turn != turn_id {
//             turn = cvar.wait(turn).unwrap();
//         }
//         // player with matching mutex id proceeds

//         // get the action for each player
//         {
//             let mut dealer = dealer_ptr.lock().unwrap();
//             conduct_action(&mut *dealer, player, socket);
//         }

//         // increase turn id by 1 for the next player
//         *turn += 1;
//         // notify everone still asleep/waiting so next player proceeds
//         cvar.notify_all();
//     }

//     // double barrier required to ensure inner code runs before next sections
//     barrier.wait();
//     // reset turn to 0
//     {
//         let mut turn = mutex.lock().unwrap();
//         *turn = 0;
//     }
//     barrier.wait();


//     // --- allow players to discard cards (according to id) one at a time ---
//     {
//         let mut turn = mutex.lock().unwrap();

//         // if the player's turn id does not match mutex num, wait
//         while *turn != turn_id {
//             turn = cvar.wait(turn).unwrap();
//         }
//         // player with matching mutex id proceeds

//         // let each player discard cards
//         player_discard_cards(player, socket);

//         // increase turn id by 1 for the next player
//         *turn += 1;
//         // notify everone still asleep/waiting so next player proceeds
//         cvar.notify_all();
//     }
// }


// fn dealing_phase(
//     dealer_ptr: &mut Arc<Mutex<FiveCardDrawDealer>>,
//     player: &mut PokerPlayer,
//     socket: &mut TcpStream,
// ) {
//     // dealer deals missing cards to players
//     {
//         let mut dealer = dealer_ptr.lock().unwrap();
//         deal_cards_to_player(&mut *dealer, player);
//     }

//     // send players their hands
//     send_hand(&player, socket);
// }