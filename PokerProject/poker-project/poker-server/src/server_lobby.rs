/// portion of server that handles client lobbies

use crate::{
    server_utils::ServerState,
    server_communication,
};

use std::{
    collections::HashMap,
    net::{TcpStream},
    sync::{mpsc, Arc, Mutex, Condvar},
    thread,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use poker_model::{
    game_types::{
        dealer_game::DealerGameTypes,
    },
};
use database_handler::DatabaseHandler;


enum ServerLobbyState {
    MainMenuScreen,
    PromptGameTypeScreen,
    WaitingForGameScreen(DealerGameTypes),
    CheckStatisticsScreen,
    Exit, // To stop the loop
}

/// This struct provides the functionality for handling a client wishes to connect/register
pub struct ServerLobbyHandler;

impl ServerLobbyHandler {
    /// Handles client lobbies (one lobby for each client)
    pub fn handle_client_lobbies(
        clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        clients_votes: Arc<Mutex<HashMap<String, DealerGameTypes>>>,
        num_players: Arc<Mutex<usize>>,
        database: Arc<Mutex<DatabaseHandler>>,
    ) {
        // client lobby thread stores handles
        let mut client_lobby_thread_handles = Vec::new();
        
        // continuously loop through connected clients
        // TODO: add delay to reduce polling issues
        loop {
            {
                let mut clients_locked = clients.lock().unwrap();
                // get each client connection
                for (client_name, client_data) in clients_locked.iter_mut() {
                    let (client_id, client_socket, client_status) = client_data;

                    // if they are in lobby or game, do not do anything
                    if *client_status == ServerState::Lobby
                        || *client_status == ServerState::InGame {
                        continue;
                    }

                    // else client has just connected, spawn a new lobby
                    // set their status to Lobby
                    *client_status = ServerState::Lobby;

                    // increase number of players in lobby by 1
                    {
                        let mut num_players_locked = num_players.lock().unwrap();
                        *num_players_locked += 1;
                    }

                    // clone their name, socket, hashmap of client votes for the game, and the database
                    let clients_clone = Arc::clone(&clients);
                    let client_name_clone: String = client_name.clone();
                    let clients_votes_clone = Arc::clone(&clients_votes);
                    let mut client_socket_clone: TcpStream = client_socket.try_clone().unwrap();
                    let num_players_lobby_clone = Arc::clone(&num_players);
                    let db_ptr = Arc::clone(&database);

                    // spawn a lobby thread for the client
                    let thread_single_client_lobby = thread::spawn(move || {
                        // lobby running for client
                        Self::create_lobby(clients_clone, client_name_clone, clients_votes_clone, &mut client_socket_clone, db_ptr);

                        // when here, players have exited the lobby.
                        // decrease number of players in lobby by 1
                        {
                            let mut num_players_lobby_locked = num_players_lobby_clone.lock().unwrap();
                            *num_players_lobby_locked -= 1;
                        }
                    });

                    // store Job Handler
                    client_lobby_thread_handles.push(thread_single_client_lobby);
                }
            }
        }

        // manager thread waits
        for thread in client_lobby_thread_handles.drain(..) {
            thread.join().unwrap();
        }
        // clear the thread handles
        client_lobby_thread_handles.clear();
    }

    /// Create and manage lobby for socket
    pub fn create_lobby(
        clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        client_name: String,
        clients_votes: Arc<Mutex<HashMap<String, DealerGameTypes>>>,
        socket: &mut TcpStream,
        db: Arc<Mutex<DatabaseHandler>>
    ) {
        // welcome player to the lobby
        let message_barrier: &str = "-----------\n";
        let lobby_welcome_message: String = format!("Welcome to the lobby player {}", client_name);

        server_communication::write_message(socket, message_barrier);
        server_communication::write_message(socket, &lobby_welcome_message);
        server_communication::write_message(socket, message_barrier);

        // begin at main menu
        let mut nextLobbyState: ServerLobbyState = Self::main_menu_screen(socket);
        loop {
            match nextLobbyState {
                // main menu screen
                ServerLobbyState::MainMenuScreen => {
                    nextLobbyState = Self::main_menu_screen(socket);
                }
                // prompt game type screen
                ServerLobbyState::PromptGameTypeScreen => {
                    nextLobbyState = Self::prompt_game_screen(socket);
                }
                // run game screen
                ServerLobbyState::WaitingForGameScreen(valid_game) => {
                    // clone data to be passed inside
                    let client_name_clone: String = client_name.clone();
                    let clients_clone = Arc::clone(&clients);
                    let clients_votes_clone = Arc::clone(&clients_votes);
                    nextLobbyState = Self::waiting_for_game_screen(socket, client_name_clone, clients_clone, clients_votes_clone, valid_game);

                }
                // statistics screen
                ServerLobbyState::CheckStatisticsScreen => {
                    // clone data to be passed inside
                    let client_name_clone: String = client_name.clone();
                    let db_clone = Arc::clone(&db);
                    nextLobbyState = Self::check_statistics_screen(client_name_clone, socket, db_clone);
                }
                // exit lobby
                ServerLobbyState::Exit => {
                    let message: &str = "Leaving lobby.";
                    server_communication::write_message(socket, message);
                    break;
                }
            }
        }

        return;
    }

    /// Show the main menu screen (menu option of possible things to do)
    pub fn main_menu_screen(socket: &mut TcpStream) -> ServerLobbyState {
        let message: &str = 
            "Choose an action to take by number (1-3):\n\
            1. Play a game\n\
            2. View player statistics\n\
            3. Exit\n\
        ";
        loop {
            // --- send message of possible options ---
            server_communication::write_message(socket, message);
            
            // --- receive user option ---
            let response: String = server_communication::read_message_wait(socket);
            let response_num: u8 = response.parse::<u8>().unwrap();
            match response_num {
                1 => {
                    return ServerLobbyState::PromptGameTypeScreen;
                }
                2 => {
                    return ServerLobbyState::CheckStatisticsScreen;
                }
                3 => {
                    return ServerLobbyState::Exit;
                }
                _ => {
                    let error_message: &str = "Invalid response.";
                    server_communication::write_message(socket, error_message);
                }
            }
        }
    }

    /// Get a client's vote by prompting them on which poker variant they want to play
    ///
    /// Parameters:
    /// socket - TcpStream: TcpStream socket for client
    ///
    /// Returns:
    /// ServerLobbyState: Next state to go to
    pub fn prompt_game_screen(socket: &mut TcpStream) -> ServerLobbyState {
        let game_list = DealerGameTypes::iter()
            .map(|game_type| format!("- {:?}", game_type))
            .collect::<Vec<_>>()
            .join("\n");

        
        let message: String = format!(
            "Which Poker variant would you like to play?:
            \n{}
            Type 'E', 'e', or 'exit' to leave.
            ", game_list
        );

        // --- loop until we receive a proper response ---
        loop {
        // --- send message of possible poker variants ---
        server_communication::write_message(socket, &message);

            // Wait and get a response
            let response: String = server_communication::read_message_wait(socket);

            // Check if the user wants to exit
            if matches!(response.to_lowercase().as_str(), "e" | "exit") {
                return ServerLobbyState::MainMenuScreen;
            }
            // Check if response matches a valid game type
            else if let Ok(valid_game) = response.parse::<DealerGameTypes>() {
                return ServerLobbyState::WaitingForGameScreen(valid_game);
            }
            else {
                // Invalid response, reprompt the client
                let invalid_message: &str = "Invalid poker variant given.";
                server_communication::write_message(socket, invalid_message);
            }
        }
    }

    /// Update client status to be waiting for a game
    ///
    /// Returns:
    /// ServerLobbyState: Next state to go to
    pub fn waiting_for_game_screen(
        socket: &mut TcpStream,
        client_name: String,
        clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        clients_votes: Arc<Mutex<HashMap<String, DealerGameTypes>>>,
        game_type: DealerGameTypes,
    ) -> ServerLobbyState {
        // confirm to the clients the game they decided to play
        let message: String = format!("You have decided to play {}", game_type.to_string());
        server_communication::write_message(socket, &message);

        // update the clients hashmap to set their status to InGame (actually waiting for game)
        {
            let mut clients_locked = clients.lock().unwrap();
            // Find the client in the HashMap
            if let Some(client_data) = clients_locked.get_mut(&client_name) {
                // set their status to be in game
                // TODO: change since the game thread should be the one to set client status to be in game
                client_data.2 = ServerState::InGame;
            } 
        }

        // update the clients vote hashmap to show which game they voted for
        {
            let mut clients_votes_locked = clients_votes.lock().unwrap();
            // Insert or update the client's vote
            clients_votes_locked.insert(client_name.clone(), game_type);
        }

        // exit the lobby
        // TODO: this means when a client votes, they immediately disconnect from the lobby
        // maybe find a way so they can choose to reverse their choice
        return ServerLobbyState::Exit;
    }

    /// Show the client their statistics, then go back to the lobby
    ///
    /// Returns:
    /// ServerLobbyState: Next state to go to
    pub fn check_statistics_screen(
        client_name: String,
        socket: &mut TcpStream,
        db: Arc<Mutex<DatabaseHandler>>
    ) -> ServerLobbyState {
        let result;

        // open the database
        {
            let db_unlocked = db.lock().unwrap();
            // get the player's data
            result = db_unlocked.get_player(&client_name);
        }

        // print the results
        match result {
            Ok(Some(values)) => {
                // get the individual numbers
                let (games_won, games_lost) = values;
                // write a message
                let message: String = format!(
                    "Player {} data.\n\
                    Games won: {}\n\
                    Games lost: {}\n",
                    client_name, games_won, games_lost
                );
                server_communication::write_message(socket, &message);
            }
            Ok(None) => {
                println!("check_statistics_screen: {} not found!", client_name);
            }
            Err(_) => {
                println!("check_statistics_screen: Error retrieving results for {}", client_name);
            }
        }

        return ServerLobbyState::MainMenuScreen;
    }
}
