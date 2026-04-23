/// utility functions for ALL poker games and ALL players

use std::{
    net::{TcpStream},
};

use crate::server_communication;

use poker_model::{
    game_types::dealer_game::DealerGame,
    poker_players::poker_player_base::{PokerPlayer, PlayerStatus},
    poker_deck::{
        Card,
        create_card_message,
        discard_cards,
    },
    hand_evaluator::HandEvaluator,
};

/// get the number of players who will participate in the current round
pub fn num_active_players<P: PokerPlayer>(players: &Vec<(u8, P, TcpStream)>) -> usize {
    let mut num: usize = 0;
    for (_, player, _) in players.iter() {
        if player.get_status() == PlayerStatus::Active {
            num += 1;
        }
    }
    return num;
}

/// send a player their hands
pub fn send_hand<P: PokerPlayer>(player: &P, socket: &mut TcpStream) {
    let message_barrier: &str = "-----------\n";
    let players_cards_msg = create_card_message(&player.get_hand());
    let message: String = format!("Your cards are: {}", players_cards_msg);
    server_communication::write_message(socket, message_barrier);
    server_communication::write_message(socket, &message);
    server_communication::write_message(socket, message_barrier);
}

/// inform player how many chips they have
pub fn player_inform_chips<P: PokerPlayer>(players: &mut Vec<(u8, P, TcpStream)>) {
    let message_barrier: &str = "-----------\n";
    for (_, player, socket) in players.iter_mut() {
        let player_msg: String = format!("\nAfter the previous action, you now have {} chips left!\n", player.get_num_chips());
        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, &player_msg);
        server_communication::write_message(socket, message_barrier);
    }
}

/// player pays the dealer
pub fn player_pays_dealer<D: DealerGame, P: PokerPlayer>(dealer: &mut D, player: &mut P, amount: usize) {
    let player_remaining_chips = player.pay(amount);
    dealer.add_to_pot(amount);
}

/// players pay the dealer's ante
pub fn players_pay_ante<D: DealerGame, P: PokerPlayer>(
    dealer: &mut D,
    players: &mut Vec<(u8, P, TcpStream)>
) {
    let message_barrier: &str = "-----------\n";
    let need_to_pay_msg: String = format!("\nYou need to pay the dealer the ante of {} to participate in this round!\n", dealer.get_ante());

    for (_, player, socket) in players.iter_mut() {
        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, &need_to_pay_msg);

        // only pay if active
        // NOTE: active players are players with chips
        // however, if they don't have enough money to pay ante, they just go all in
        if player.get_status() == PlayerStatus::Active {
            let paying_dealer_msg: &str = "Paying the dealer.\n";

            player_pays_dealer(dealer, player, dealer.get_ante());

            server_communication::write_message(socket, message_barrier);
            server_communication::write_message(socket, paying_dealer_msg);

            // if player has no more chips, set status to all in
            if player.get_num_chips() == 0 {
                let all_in_msg: &str = "You had to spend all your chips to pay the ante. Be careful!\n";
                player.set_status(PlayerStatus::AllIn);

                server_communication::write_message(socket, message_barrier);
                server_communication::write_message(socket, all_in_msg);
            }
        }
        // player is eliminated, remains on the sidelines
        else if player.get_status() == PlayerStatus::Eliminated {
            let eliminated_msg: &str = "Sorry, you don't have the money to pay the dealer.\n";

            server_communication::write_message(socket, message_barrier);
            server_communication::write_message(socket, eliminated_msg);
        }

        server_communication::write_message(socket, message_barrier);
    }
}

/// write to all players the current round, how many chips they have, and their status as a result (Active or Eliminated)
pub fn player_status_report<D: DealerGame, P: PokerPlayer>(dealer: &mut D, players: &mut Vec<(u8, P, TcpStream)>) {
    let message_barrier: &str = "-----------\n";
    let dealer_round_msg: String = format!("Welcome to round {}.\n", dealer.get_current_round());

    for (_, player, socket) in players.iter_mut() {
        let mut player_msg = String::new();
        if player.get_status() == PlayerStatus::AllIn {
            player_msg = format!("\nYou have 0 chips left, All in!\n");
        }
        else if player.get_status() == PlayerStatus::Eliminated {
            player_msg = format!("\nYou have 0 chips left, and are therefore eliminated. Sorry!\n");
        }
        else {
            player_msg = format!("\nYou have {} chips left, and are still in the game!\n", player.get_num_chips());
        }

        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, &dealer_round_msg);
        server_communication::write_message(socket, &player_msg);
        server_communication::write_message(socket, message_barrier);
    }
}

/// conduct betting phase in a round
pub fn betting_phase<D: DealerGame, P: PokerPlayer>(
    dealer: &mut D,
    players: &mut Vec<(u8, P, TcpStream)>,
    num_players_in_betting_round: &mut usize,
) {
    // determines if player can check (can check every new betting round)
    let mut can_check: bool = true;
    
    // resets current bet to prevent excess chips being wagered when re-raising
    for (_, player, _) in players.iter_mut() {
        player.set_contributed_chips(0);
    }
    dealer.set_current_bet(0);

    // number of players that Check/Call/Raise in sequence
    let mut num_players_that_took_action: usize = 0;
    
    // if the number of players that still need to take an action == number of players that took an action (Check/Call/Raise),
    // we can end the betting loop
    while num_players_that_took_action < *num_players_in_betting_round {
        // go through each player sequentially
        for (_, player, socket) in players.iter_mut() {
            // tackles scenario where n players enter betting round, 
            // - n-1 are all-in
            // - 1 is folded
            // tackles scenario where all players fold (which is not allowed)
            if *num_players_in_betting_round == 1 {
                // if num_players_that_took_action > 0 && *num_players_in_betting_round == 1 {
                return; // exit before prompting last player
            }

            // if player has folded, is all in, or is eliminated (no chips), skip them
            if player.get_status() == PlayerStatus::Folded
            || player.get_status() == PlayerStatus::AllIn
            || player.get_status() == PlayerStatus::Eliminated {
                continue;
            }

            // player can Check, Call, Raise, or Fold
            conduct_action(dealer, player, socket, can_check);

            match player.get_status() {
                // if the player checked, increase the count of players that took an action
                PlayerStatus::Checked => num_players_that_took_action += 1,

                // If the player called, increase the count of players that took an action
                // subsequent players cannot check
                PlayerStatus::Called => {
                    num_players_that_took_action += 1;
                    can_check = false;
                },
                
                // if the player raised, reset the count of players who haven't raised
                // subsequent players cannot check
                PlayerStatus::Raised => {
                    num_players_that_took_action = 1; // reset to 0, then +1 since player took action
                    can_check = false;
                },

                // if the player folded, decrease number of players required to take an action
                PlayerStatus::Folded => *num_players_in_betting_round -= 1,

                // other status i.e active, sitting out not considered.
                _ => panic!("No other status allowed."),
            };

            server_communication::write_message(socket, "--- Action received. ---\n");

            // additionally, if player is all in (no chips), they cannot take an action.
            // reduce number of players needed to take an action and set their status to all in
            if player.get_num_chips() == 0 {
                num_players_that_took_action = 1;
                *num_players_in_betting_round -= 1;
                player.set_status(PlayerStatus::AllIn);
            }

            // break out of `for` loop early if condition is met.
            // for example, with 2 players, if player2 Raises, and player1 Checks, no need to go backt to player2
            if (num_players_that_took_action >= *num_players_in_betting_round) {
                break;
            }
        }
    }
}

/// player can Check, Call, Raise, or Fold
pub fn conduct_action<D: DealerGame, P: PokerPlayer>(
    dealer: &mut D,
    player: &mut P,
    socket: &mut TcpStream,
    can_check: bool,
) {
    // messages
    let mut initial_message: String = String::new();
    let mut invalid_message: &str = "";

    if can_check {
        initial_message = format!("The current pot is {}. There is currently no bet. What would you like to do? Options are Check, Raise, Fold.\n", dealer.get_pot());
        invalid_message = "Invalid choice. Options are Check, Call, Raise, Fold.\n";
    }
    else {
        initial_message = format!("The current pot is {}. The current bet is {}. What would you like to do? Options are Call, Raise, Fold.\n", dealer.get_pot(), dealer.get_current_bet());
        invalid_message = "Invalid choice. Options are Call, Raise, Fold.\n";
    }

    // loop in case they have an invalid response i.e want to raise but not enough chips to raise
    loop {
        // ask them what they want to do
        server_communication::write_message(socket, &initial_message);

        // get a valid response for which action they want to do
        let response: String = loop {
            let response = server_communication::read_message_wait(socket);
        
            // player allowed to check
            if can_check {
                if response.as_str() == "Check"
                // if player can check, then they shouldn't be able to call as they currently are matching the pot
                // || response.as_str() == "Call"
                || response.as_str() == "Raise"
                || response.as_str() == "Fold" {
                    break response;
                }
                else {
                    // invalid response, reprompt the client
                    server_communication::write_message(socket, invalid_message);
                }
            }
            // player not allowed to check
            else {
                // check if response matches a valid type
                if response.as_str() == "Call"
                || response.as_str() == "Raise"
                || response.as_str() == "Fold" {
                    break response;
                }
                else {
                    // invalid response, reprompt the client
                    server_communication::write_message(socket, invalid_message);
                }
            }
        };
        
    
        // do something based on response
        match response.as_str() {
            "Check" => {
                // do nothing
                player.set_status(PlayerStatus::Checked);
                break;
            },
            "Call" => {
                // player calls, and is either checked or all in
                // let diff = current_bet.saturating_sub(player.get_current_bet());
                // let paid = player.call(*current_bet); // will only deduct the diff
                // dealer.add_to_pot(paid);

                // NOTE: we assume a player has enough chips when calling this code

                // when a player calls, They only need to pay the difference between
                // the raised value,
                // and the current number of chips they contributed last time they paid
                let diff: usize = dealer.get_current_bet() - player.get_contributed_chips();
                player_pays_dealer(dealer, player, diff);

                // update the number of chips the player contributed to match the raise
                player.set_contributed_chips(dealer.get_current_bet());
                
                player.set_status(PlayerStatus::Called);
                break;
            },
            "Raise" => {
                // check if enough to raise. If not enough, reprompt
                if player.get_num_chips() <= dealer.get_current_bet() {
                    let not_enough_chips_message: String = format!("You have too few chips to raise. You have {} chips, but the current bet is {}.\n", player.get_num_chips(), dealer.get_current_bet());
                    server_communication::write_message(socket, &not_enough_chips_message);
                    continue;
                }
                
                // messages
                let bet_message: String = format!("How much do you want to bet? Current bet is {}: \n", dealer.get_current_bet());
                let mut invalid_raise_message: &str = "";

                // variables to check if player can raise
                let mut proposed_amount_valid: bool = false;
                let mut enough_chips: bool = false;
    
                // ask how much to bet
                server_communication::write_message(socket, &bet_message);

                loop {
                    let response = server_communication::read_message_wait(socket);

                    // check if response matches a valid type
                    if let Ok(amount) = response.trim().parse::<usize>() {

                        // proposed amount must be greater than dealer's current bet
                        if amount > dealer.get_current_bet() {
                            proposed_amount_valid = true;
                        }
                        else {
                            invalid_raise_message = "Invalid choice. Please raise by a value higher than the current ante.\n";
                        }

                        // player must have enough chips to meet the proposed bet
                        if player.get_num_chips() >= amount {
                            enough_chips = true;
                        }
                        else {
                            invalid_raise_message = "Invalid choice. You have too few chips to bet.\n";
                        }

                        // both conditions met
                        if proposed_amount_valid && enough_chips {
                            // update dealer's bet to the new bet
                            dealer.set_current_bet(amount);
                            // player pays new bet
                            player_pays_dealer(dealer, player, amount);
                            // break out of the loop
                            break;
                        }

                        // if amount > dealer.get_ante() && amount >= player.get_current_bet() {
                        // if amount >= player.get_current_bet() {
                        //     if player.get_num_chips() >= amount.saturating_sub(player.get_current_bet()) {
                        //         *current_bet = amount;
                        //         dealer.set_current_bet(*current_bet);
                        //         let paid = player.call(*current_bet);
                        //         dealer.add_to_pot(paid);
                        //         player.set_status(PlayerStatus::Raised);

                        //         break;
                        //     }
                        //     else {
                        //         server_communication::write_message(
                        //             socket,
                        //             "You don't have enough chips to raise that amount.\n",
                        //         );
                        //     }
                        // }
                        // else {
                        //     server_communication::write_message(
                        //         socket,
                        //         "Raise must be higher than the current bet.\n",
                        //     );
                        // }
                    }
                    else {
                        server_communication::write_message(
                            socket,
                            "Invalid number. Please enter a valid integer.\n",
                        );
                    }
                }
                // set player status to raise
                player.set_status(PlayerStatus::Raised);
                break;
            },            
            "Fold" => {
                // do nothing
                player.set_status(PlayerStatus::Folded);

                break;
            },
            _ => {
                panic!("Error");
            },
        };
    }
}

/// evaluate winner per round
pub fn evaluate_winner<P: PokerPlayer>(players: &mut Vec<(u8, P, TcpStream)>) -> (Vec<String>, Vec<Card>) {
    // holds the final hands
    let mut final_hands: Vec<(String, Vec<Card>)> = Vec::new();

    // collect all hands from players that didn't fold or are eliminated
    for (_, player, socket) in players.iter() {
        if player.get_status() == PlayerStatus::Folded
        || player.get_status() == PlayerStatus::Eliminated {
            continue;
        }
        
        // push the player's name, and the best 5-hand from the player's total possible hand
        final_hands.push((player.get_name(), HandEvaluator::best_five_hand_from_larger(player.get_hand())));
    }

    // evaluate winning player and hand
    let (winners, hands) = HandEvaluator::rank_hands(final_hands);
    let names: String = winners.join(", ");

    // send message to all players who the winning hand is
    inform_players_of_winner(players, &names, &hands);

    // return the name(s) and hand(s) of the winner(s)
    return (winners, hands);
}

/// evaluate winners: special case where all other players folded
pub fn all_fold<P: PokerPlayer>(players: &mut Vec<(u8, P, TcpStream)>) -> (Vec<String>, Vec<Card>) {
    // Find the last remaining player
    let winner_name = players.iter()
        .find(|(_, p, _)| (p.get_status() != PlayerStatus::Folded) && (p.get_status() != PlayerStatus::Eliminated))
        .map(|(_, p, _)| p.get_name().to_string()) 
        .unwrap();

    let winner_names = vec![winner_name];
    let winner_hands = Vec::<Card>::new(); 

    inform_players_of_winner(players, &winner_names[0], &winner_hands);

    return (winner_names, winner_hands)
}

/// send message to all players who the winning hand is
// NOTE: use rayon to speed things up via par_iter_mut?
pub fn inform_players_of_winner<P: PokerPlayer>(
    players: &mut Vec<(u8, P, TcpStream)>,
    winner_names_combined: &str,
    winner_hands_combined: &Vec<Card>,
) {
    let message_barrier: &str = "-----------\n";

    for (_, player, socket) in players.iter_mut() {
        let players_cards_msg = create_card_message(&winner_hands_combined);
        let msg: String = format!(
            "The round is over!\nWinner(s): {}\nWinning Hand(s): {}",
            winner_names_combined, players_cards_msg
        );
        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, &msg);
        server_communication::write_message(socket, message_barrier);
    }
}

/// distribute dealer's pot across players
/// rule: if 1 winner, give that winner the pot
/// rule: if multiple winners, all split the pot evenly. The remainder is distributed across the first players in the vector
pub fn distribute_pot<D: DealerGame, P: PokerPlayer>(dealer: &mut D, players: &mut Vec<(u8, P, TcpStream)>, winner_names: &Vec<String>) {
    // only 1 winner
    if winner_names.len() == 1 {
        for (_, player, _) in players.iter_mut() {
            if winner_names.contains(&player.get_name()) {
                // give the player the entire pot and break (only 1 player)
                player.receive_chips(dealer.get_pot());
                break;
            }
        }
    }
    // 2+ winners, distribute pot evenly
    else {
        let pot = dealer.get_pot();
        let split: usize = pot / winner_names.len();
        let mut remainder: usize = pot % winner_names.len();

        for (_, player, _) in players.iter_mut() {
            if winner_names.contains(&player.get_name()) {
                // give the player a split
                player.receive_chips(split);

                // give the player a chip from the remainder if there still is
                if remainder > 0 {
                    player.receive_chips(1);
                    remainder -= 1;
                }
            }
        }
    }

    // clear the dealer pot
    dealer.clear_pot();
}

/// check if there is only 1 winner. If there is, return true (game has been won) and do nothing
/// else, return false (game continue still) and reset players and dealers for a new round (clear hands and reshuffle the deck)
pub fn go_to_next_round_and_set_environment<D: DealerGame, P: PokerPlayer>(dealer: &mut D, players: &mut Vec<(u8, P, TcpStream)>) -> bool {
    let mut num_active: u8 = 0;
    // reset all players status and clear their hands
    for (_, player, _) in players.iter_mut() {
        if player.get_num_chips() > 0 {
            player.set_status(PlayerStatus::Active);
            num_active += 1;
        }
        else {
            player.set_status(PlayerStatus::Eliminated);
        }

        player.set_contributed_chips(0);
        player.clear_hand();
    }

    // if num_active == 1 then only 1 winner.
    if num_active == 1 {
        return false;
    }
    
    // reset deck, current_bet, pot
    dealer.reset_deck();
    dealer.shuffle_deck();
    dealer.clear_pot();
    dealer.set_current_bet(0);
    // increment for next round
    dealer.next_round();

    return true;
}

/// inform players of game end
pub fn inform_players_game_ended<P: PokerPlayer>(players: &mut Vec<(u8, P, TcpStream)>) {
    let message_barrier: &str = "-----------\n";
    let msg_game_end: &str = "Game has ended. Thank you for playing, and gamble responsibly!\n";
    for (_, _, socket) in players.iter_mut() {
        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, msg_game_end);
        server_communication::write_message(socket, message_barrier);
    }
}