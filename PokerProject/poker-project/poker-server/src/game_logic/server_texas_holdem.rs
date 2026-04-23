/// contains game logic progression for texas hold'em

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
        texas_holdem::TexasHoldemDealer,
    },
    poker_players::{
        poker_player_base::{PokerPlayer, PlayerStatus},
        poker_player_texas_holdem::PokerPlayerTexasHoldem,
    },
    poker_deck::{
        Card,
        discard_cards,
        create_card_message,
    },
    hand_evaluator::HandEvaluator,
};

// function that begins a texas hold'em game
pub fn host_texas_holdem(
    players: &mut Vec<(u8, PokerPlayerTexasHoldem, TcpStream)>,
) -> String {
    let mut dealer = TexasHoldemDealer::new();
    dealer.setup();

    // variable that determines if we end the game or not
    let mut continue_playing: bool = true;

    // stores current winners
    let mut winner_names: Vec<String> = Vec::new();
    let mut winner_hands: Vec<Card> = Vec::new();
    
    while continue_playing {
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
        // community cards
        dealer.clear_community_cards();

        for round_num in 0..4 {
            if round_num == 0 {
                // --- Deal 2 hole cards to each player ---
                for (_, player, socket) in players.iter_mut() {
                    let hole_cards = dealer.deal_cards(2);
                    player.receive_cards(hole_cards);
                    send_hand(player, socket);
                }
            }

            if round_num > 0 {
                if round_num == 1 {
                    dealer.deal_community_cards(3);
                } else {
                    dealer.deal_community_cards(1);
                }
            broadcast_community_cards(&dealer, players);
            }

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
            
            if num_players_in_betting_round < 2 && !all_in_player {
                break;
            }
        }
    
        if num_players_in_betting_round > 1 || all_in_player {
            let mut final_hands: Vec<(String, Vec<Card>)> = Vec::new();
            
            for (_, player, _) in players.iter() {
                if player.get_status() == PlayerStatus::Folded
                || player.get_status() == PlayerStatus::Eliminated {
                    continue;
                }
    
                let best_hand = player.best_hand_with_community(dealer.get_community_cards());
                final_hands.push((player.get_name(), best_hand));
            }
        
            (winner_names, winner_hands) = HandEvaluator::rank_hands(final_hands);
            // let names = winner_names.join(", ");
            // inform_players_of_winner(players, &names, &winner_hands);

        } else {
            (winner_names, winner_hands) = all_fold(players);
        }
        
        let names = winner_names.join(", ");
        inform_players_of_winner(players, &names, &winner_hands);      
        
        // distribute Pot evenly
        distribute_pot(&mut dealer, players, &winner_names);

        continue_playing = go_to_next_round_and_set_environment(&mut dealer, players);
    }
    
    // inform players that game has ended
    inform_players_game_ended(players);
    
    // return winner
    return winner_names[0].clone();
}

fn broadcast_community_cards(
    dealer: &TexasHoldemDealer,
    players: &mut Vec<(u8, PokerPlayerTexasHoldem, TcpStream)>,
) {
    let community = dealer.get_community_cards();
    let msg = format!(
        "Community cards: {}\n",
        create_card_message(&community)
    );

    for (_, _, socket) in players.iter_mut() {
        server_communication::write_message(socket, &msg);
    }
}

