/// contains game logic progression for seven card stud

use std::{
    collections::HashMap,
    net::{TcpStream},
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
        seven_card_stud::SevenCardStudDealer,
    },
    poker_players::{
        poker_player_base::{PokerPlayer, PlayerStatus},
        poker_player_seven_card_stud::PokerPlayerSevenCardStud,
    },
    poker_deck::{
        Card,
        discard_cards,
        create_card_message,
    },
    hand_evaluator::HandEvaluator,
};

// function that begins a seven card stud game
pub fn host_seven_card_stud(
    players: &mut Vec<(u8, PokerPlayerSevenCardStud, TcpStream)>,
) -> String {
    // create and setup the dealer
    let mut dealer: SevenCardStudDealer = SevenCardStudDealer::new();
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
        
        // 5 mini-rounds (0-4)
        for round_num in 0..5 {
            // --- deal cards phase ---
            // only deal if there are 2+ players that are still in the round
            if num_players_in_betting_round > 1 {
                deal_cards_phase(&mut dealer, players, round_num);
            }

            // send players their hands
            send_hand_phase(players);

            // --- shift player vector around to properly iterate over them ---
            determine_player_order(players, round_num);

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
        }

        // --- evaluate the winner for the current hand (only if 2+ people didn't fold) --- 
        if num_players_in_betting_round > 1 {
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

    // return the winners
    return winner_names[0].clone();
}

// determine which player goes first, and shift player vector to match new order
fn determine_player_order(
    players: &mut Vec<(u8, PokerPlayerSevenCardStud, TcpStream)>,
    round_num: usize,
) {
    // rank the hands to find the player who goes first
    let mut players_winning_hand_name: Vec<String> = Vec::new();
    let mut players_winning_hand: Vec<Card> = Vec::new();
    // holds the current face up hands
    let mut current_face_up_hands: Vec<(String, Vec<Card>)> = Vec::new();

    // collect all player hands
    for (_, player, _) in players.iter() {
        // if player has folded or is eliminated, skip them
        if player.get_status() == PlayerStatus::Folded
        || player.get_status() == PlayerStatus::Eliminated {
            continue;
        }

        // store the hand
        current_face_up_hands.push((player.get_name(), player.get_face_up_cards()));
    }

    // - if initial round, player with worst face up hand begins
    if (round_num == 0) {
        (players_winning_hand_name, players_winning_hand) = HandEvaluator::rank_hands(current_face_up_hands.clone());
    }
    // - not initial round as round_num > 0, so player with best face up hand begins
    else {
        (players_winning_hand_name, players_winning_hand) = HandEvaluator::rank_hands(current_face_up_hands.clone());
    }

    // rotate vector until chosen player is at index 0
    if let Some(start_index) = players.iter().position(|(_, player, _)| player.get_name() == players_winning_hand_name[0]) {
        players.rotate_left(start_index);
    }
}

// send a player their hands
fn send_player_their_hand_seven_card_draw(player: &PokerPlayerSevenCardStud, socket: &mut TcpStream) {
    let player_face_down_cards_msg: String = create_card_message(&player.get_face_down_cards());
    let player_face_up_cards_msg: String = create_card_message(&player.get_face_up_cards());
    let message_barrier: &str = "-----------\n";
    let face_down_message: String = format!("Your private (face-down) cards are: {}\n", player_face_down_cards_msg);
    let face_up_message: String = format!("Your public (face-up) cards are: {}\n", player_face_up_cards_msg);

    server_communication::write_message(socket, message_barrier);
    server_communication::write_message(socket, &face_down_message);
    server_communication::write_message(socket, &face_up_message);
    server_communication::write_message(socket, message_barrier);
}

// send all players still in the game their hands
fn send_hand_phase(players: &mut Vec<(u8, PokerPlayerSevenCardStud, TcpStream)>) {
    // name + face-up-cards-message
    let mut face_up_card_messages: HashMap<String, String> = HashMap::new();

    // iterate through each player
    for (_, player, socket) in players.iter_mut() {
        // if player has folded or is eliminated, skip them
        if player.get_status() == PlayerStatus::Folded
        || player.get_status() == PlayerStatus::Eliminated {
            continue;
        }
        
        // send a player their specific hand
        send_player_their_hand_seven_card_draw(&player, socket);

        // store their face up cards in a message
        let player_face_up_cards_msg: String = create_card_message(&player.get_face_up_cards());
        let face_up_message: String = format!("{}'s public (face-up) cards are: {}\n", player.get_name(), player_face_up_cards_msg);
        face_up_card_messages.insert(player.get_name(), face_up_message);
    }

    // iterate through each player again
    for (_, player, socket) in players.iter_mut() {
        // iterate through all the face up card messages
        for (name, message) in face_up_card_messages.iter() {
            // if the player name does not match the name attached to that message
            // send the message to the player
            if player.get_name() != *name {
                let message_barrier: &str = "-----------\n";
                server_communication::write_message(socket, message_barrier);
                server_communication::write_message(socket, &message);
                server_communication::write_message(socket, message_barrier);
            }
        }
    }
}

// deal cards to players
fn deal_cards_phase(
    dealer: &mut SevenCardStudDealer,
    players: &mut Vec<(u8, PokerPlayerSevenCardStud, TcpStream)>,
    round_num: usize,
) {
    for (_, player, _) in players.iter_mut() {
        // if player has folded or is eliminated, skip them
        if player.get_status() == PlayerStatus::Folded
        || player.get_status() == PlayerStatus::Eliminated {
            continue;
        }
        
        // 1st round
        if (round_num == 0) {
            // deal 2 facedown and 1 faceup
            player.receive_face_down_cards(dealer.deal_cards(2));
            player.receive_face_up_cards(dealer.deal_cards(1));
        }
        // 5th round
        else if (round_num == 4) {
            // deal 1 face down
            player.receive_face_down_cards(dealer.deal_cards(1));
        }
        // 2nd to 4th round
        else {
            // deal 1 face up
            player.receive_face_up_cards(dealer.deal_cards(1));
        }
    }
}