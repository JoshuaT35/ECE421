use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

use poker_model::poker_players::poker_player_texas_holdem::PokerPlayerTexasHoldem;
use poker_model::poker_players::poker_player_base::{PokerPlayer, PlayerStatus};
use poker_model::game_types::texas_holdem::TexasHoldemDealer;
use poker_model::poker_deck::{Card, create_card_message};
use poker_model::hand_evaluator::HandEvaluator;

// for integration testing
mod server_communication {
    use std::io::{Write, BufRead, BufReader};
    use std::net::TcpStream;

    pub fn write_message(stream: &mut TcpStream, msg: &str) {
        writeln!(stream, "{}", msg).unwrap();
    }

    pub fn read_message(stream: &mut TcpStream) -> String {
        let mut buf = String::new();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        reader.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}

// for integration testing
fn host_texas_holdem(players: &mut Vec<(u8, PokerPlayerTexasHoldem, TcpStream)>) -> String {
    let mut dealer = TexasHoldemDealer::new();
    dealer.setup();

    let mut continue_playing = true;
    let mut winner_names: Vec<String> = Vec::new();
    let mut winner_hands: Vec<Card> = Vec::new();

    while continue_playing {
        dealer.clear_community_cards();

        for (_, player, socket) in players.iter_mut() {
            let hand = dealer.deal_hole_cards();
            player.receive_cards(hand);
            let msg = format!("Your hand: {}\n", create_card_message(&player.get_hand()));
            server_communication::write_message(socket, &msg);
        }

        for round_num in 0..4 {
            if round_num == 0 {
            } else if round_num == 1 {
                dealer.deal_community_cards(3); 
            } else {
                dealer.deal_community_cards(1); 
            }

            broadcast_community_cards(&dealer, players);

            for (_, player, socket) in players.iter_mut() {
                if player.get_status() == PlayerStatus::Folded {
                    continue;
                }

                server_communication::write_message(socket, "Integration test message: Enter action");
                let action = server_communication::read_message(socket);

                match action.to_lowercase().as_str() {
                    "fold" => player.set_status(PlayerStatus::Folded),
                    _ => {} // treat any input as call/check
                }
            }
        }

        // Evaluate remaining hands
        let mut final_hands = vec![];
        for (_, player, _) in players.iter() {
            if player.get_status() != PlayerStatus::Folded {
                let best_hand = player.best_hand_with_community(dealer.get_community_cards());
                final_hands.push((player.get_name(), best_hand));
            }
        }

        if final_hands.is_empty() {
            // everyone folded, pick first player who didn't fold earlier
            for (_, player, _) in players.iter() {
                if player.get_status() != PlayerStatus::Eliminated {
                    winner_names = vec![player.get_name()];
                    winner_hands = player.get_hand().to_vec();
                    break;
                }
            }
        } else {
            (winner_names, winner_hands) = HandEvaluator::rank_hands(final_hands);
        }

        let names = winner_names.join(", ");
        inform_players_of_winner(players, &names, &winner_hands);

        continue_playing = false; 
    }

    for (_, _, socket) in players.iter_mut() {
        server_communication::write_message(socket, "Game over.\n");
    }

    winner_names[0].clone()
}

fn broadcast_community_cards(
    dealer: &TexasHoldemDealer,
    players: &mut Vec<(u8, PokerPlayerTexasHoldem, TcpStream)>,
) {
    let community = dealer.get_community_cards();
    let msg = format!("Community cards: {}\n", create_card_message(&community));
    for (_, _, socket) in players.iter_mut() {
        server_communication::write_message(socket, &msg);
    }
}

fn inform_players_of_winner(
    players: &mut Vec<(u8, PokerPlayerTexasHoldem, TcpStream)>,
    winner: &str,
    winning_hand: &Vec<Card>,
) {
    let msg = format!("Winner: {} with hand {}\n", winner, create_card_message(winning_hand));
    for (_, _, socket) in players.iter_mut() {
        server_communication::write_message(socket, &msg);
    }
}

fn simulated_player(mut stream: TcpStream, name: &str) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if buffer.contains("Enter action") {
                    writeln!(stream, "call").unwrap();
                } else if buffer.contains("Game over") {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

// Integration test
#[test]
fn test_host_texas_holdem_integration_test() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let handles: Vec<_> = (0..2).map(|i| {
        let addr = addr.clone();
        thread::spawn(move || {
            let stream = TcpStream::connect(addr).unwrap();
            simulated_player(stream, &format!("Player{}", i + 1));
        })
    }).collect();

    let mut players = Vec::new();
    for i in 0..2 {
        let (stream, _) = listener.accept().unwrap();
        let player = PokerPlayerTexasHoldem::new(&format!("Player{}", i + 1));
        players.push((i as u8, player, stream));
    }

    let winner = host_texas_holdem(&mut players);
    println!("Simulated game winner: {}", winner);
    assert!(winner.starts_with("Player"));

    for handle in handles {
        let _ = handle.join();
    }
}
