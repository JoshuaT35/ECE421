// portion of server that handles game running and logic

use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    server_utils::ServerState,
    server_communication,
    game_logic::server_five_card_draw::host_five_card_draw,
    game_logic::server_seven_card_stud::host_seven_card_stud,
    game_logic::server_texas_holdem::host_texas_holdem,
};

use database_handler::DatabaseHandler;

use poker_model::{
    poker_players::{
        poker_player_base::PokerPlayer,
        poker_player_five_card_draw::PokerPlayerFiveCardDraw,
        poker_player_seven_card_stud::PokerPlayerSevenCardStud,
        poker_player_texas_holdem::PokerPlayerTexasHoldem,
    },
    game_types::dealer_game::DealerGameTypes,
};

pub struct ServerGamesHandler;

impl ServerGamesHandler {
    pub fn handle_client_games(
        clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        clients_votes: Arc<Mutex<HashMap<String, DealerGameTypes>>>,
        num_players_lobby: Arc<Mutex<usize>>,
        db: Arc<Mutex<DatabaseHandler>>,
    ) {

        // server game stores thread handles for each game being played
        let mut server_game_thread_handles = Vec::new();

        // continuously check clients who voted
        loop {
            // check if 2+ clients have voted
            // check if number of players in the lobby = 0 (everyone in lobby voted)
            let game_can_be_hosted: bool = {
                let clients_votes_locked = clients_votes.lock().unwrap();
                let num_players_locked = num_players_lobby.lock().unwrap();
        
                clients_votes_locked.len() >= 2 && *num_players_locked == 0
            };
        
            // if one of the conditions is not satisfied, loop again
            // else create a game with all the players in the clients
            if !game_can_be_hosted {
                continue;
            }

            // get the majority vote
            let mut game_to_play: DealerGameTypes;
            {
                let clients_votes_locked = clients_votes.lock().unwrap();
                game_to_play = Self::determine_majority_game(&clients_votes_locked);
            }


            // --- make vector of players depending on game ---
            // for each client that voted, create a Player for them with their socket
            let mut players_five_card_draw: Vec<(u8, PokerPlayerFiveCardDraw, TcpStream)> = Vec::new();
            let mut players_texas_holdem: Vec<(u8, PokerPlayerTexasHoldem, TcpStream)> = Vec::new();
            let mut players_seven_card_stud: Vec<(u8, PokerPlayerSevenCardStud, TcpStream)> = Vec::new();
            {
                // unlock clients who voted
                let clients_vote_locked = clients_votes.lock().unwrap();
                // unlock list of connected clients
                let mut clients_locked = clients.lock().unwrap();

                // variables
                let mut player_id: u8 = 0;

                // for each client who voted
                for client_name in clients_vote_locked.keys() {
                    // get their info
                    if let Some((_, socket, state)) = clients_locked.get_mut(client_name.as_str()) {

                        match game_to_play {
                            // five card draw
                            DealerGameTypes::FiveCardDraw => {
                                let new_player: PokerPlayerFiveCardDraw = PokerPlayerFiveCardDraw::new(client_name.as_str());
                                players_five_card_draw.push((player_id, new_player, socket.try_clone().unwrap()));
                            },

                            // texas holdem
                            DealerGameTypes::TexasHoldem => {
                                let new_player: PokerPlayerTexasHoldem = PokerPlayerTexasHoldem::new(client_name.as_str());
                                players_texas_holdem.push((player_id, new_player, socket.try_clone().unwrap()));
                            }

                            // seven card stud
                            DealerGameTypes::SevenCardStud => {
                                let new_player: PokerPlayerSevenCardStud = PokerPlayerSevenCardStud::new(client_name.as_str());
                                players_seven_card_stud.push((player_id, new_player, socket.try_clone().unwrap()));
                            }

                            // another game
                            _ => println!("Game not implemented yet."),
                        }

                        // set client's state to be in-game
                        *state = ServerState::InGame;

                        // increment player_id by 1 (for the next player)
                        player_id += 1;
                    }
                }
            }

            // create thread that will host the game (match again)
            // clone the clients and db pointers to pass into the thread
            let clients_clone: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>> = Arc::clone(&clients);
            let db_clone: Arc<Mutex<DatabaseHandler>> = Arc::clone(&db);
            let thread_game = thread::spawn(
                move || {
                match game_to_play {
                    DealerGameTypes::FiveCardDraw => {
                        // host the game
                        let winner: String = host_five_card_draw(&mut players_five_card_draw);

                        for (_, player, _) in players_five_card_draw.iter_mut() {
                            let name: String = player.get_name();
                        
                            // Lock database and update stats
                            {
                                let db_locked = db_clone.lock().unwrap();
                                if let Ok(Some((games_won, games_lost))) = db_locked.get_player(&name) {
                                    if winner == name {
                                        db_locked.save_player(&name, games_won + 1, games_lost).unwrap();
                                    } else {
                                        db_locked.save_player(&name, games_won, games_lost + 1).unwrap();
                                    }
                                }
                            }
                        
                            // Lock client list and update lobby state
                            {
                                let mut clients_locked = clients_clone.lock().unwrap();
                                if let Some((_chips, _stream, state)) = clients_locked.get_mut(&name) {
                                    // Lobby means they're already in the lobby
                                    // Connection means they're going to the lobby
                                    *state = ServerState::Connection;
                                }
                            }
                        }
                    },

                    // texas holdem
                    DealerGameTypes::TexasHoldem => {
                        // host the game
                        let winner: String = host_texas_holdem(&mut players_texas_holdem);

                        for (_, player, _) in players_five_card_draw.iter_mut() {
                            let name = player.get_name();
                        
                            // Lock database and update stats
                            {
                                let db_locked = db_clone.lock().unwrap();
                                if let Ok(Some((games_won, games_lost))) = db_locked.get_player(&name) {
                                    if winner == name {
                                        db_locked.save_player(&name, games_won + 1, games_lost).unwrap();
                                    } else {
                                        db_locked.save_player(&name, games_won, games_lost + 1).unwrap();
                                    }
                                }
                            }
                        
                            // Lock client list and update lobby state
                            {
                                let mut clients_locked = clients_clone.lock().unwrap();
                                if let Some((_chips, _stream, state)) = clients_locked.get_mut(&name) {
                                    *state = ServerState::Lobby;
                                }
                            }
                        }
                    },

                    // seven card stud
                    DealerGameTypes::SevenCardStud => {
                        // host the game
                        let winner: String = host_seven_card_stud(&mut players_seven_card_stud);

                        for (_, player, _) in players_five_card_draw.iter_mut() {
                            let name = player.get_name();
                        
                            // Lock database and update stats
                            {
                                let db_locked = db_clone.lock().unwrap();
                                if let Ok(Some((games_won, games_lost))) = db_locked.get_player(&name) {
                                    if winner == name {
                                        db_locked.save_player(&name, games_won + 1, games_lost).unwrap();
                                    } else {
                                        db_locked.save_player(&name, games_won, games_lost + 1).unwrap();
                                    }
                                }
                            }
                        
                            // Lock client list and update lobby state
                            {
                                let mut clients_locked = clients_clone.lock().unwrap();
                                if let Some((_chips, _stream, state)) = clients_locked.get_mut(&name) {
                                    *state = ServerState::Lobby;
                                }
                            }
                        }
                    }

                    // another game
                    _ => println!("Game not implemented yet."),
                }
            });

            // store thread handle
            server_game_thread_handles.push(thread_game);

            // reset the hashmap of client votes
            {
                let mut clients_votes_locked = clients_votes.lock().unwrap();
                clients_votes_locked.clear();
            }
        }

        // have this function wait for the thread
        for thread in server_game_thread_handles.drain(..) {
            thread.join().unwrap();
        }
        // clear the thread handle
        server_game_thread_handles.clear();
    }

    // given a HashMap of what the clients voted for, get the game type with the most votes
    // if tied, default to FiveCardDraw
    fn determine_majority_game(
        votes: &HashMap<String, DealerGameTypes>,
    ) -> DealerGameTypes {
        let mut vote_counts: HashMap<DealerGameTypes, usize> = HashMap::new();
    
        // Count votes for each game type
        for &game in votes.values() {
            *vote_counts.entry(game).or_insert(0) += 1;
        }
    
        // Determine majority vote
        let mut max_count = 0;
        let mut game_to_play = DealerGameTypes::FiveCardDraw;
        let mut tie = false;
    
        for (&game, &count) in &vote_counts {
            if count > max_count {
                max_count = count;
                game_to_play = game;
                tie = false; // Reset tie flag
            } else if count == max_count {
                tie = true;
            }
        }
    
        if tie {
            DealerGameTypes::FiveCardDraw
        } else {
            game_to_play
        }
    }
}