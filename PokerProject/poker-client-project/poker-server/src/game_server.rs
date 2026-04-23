/// main portion of the server: ties smaller portions of the server to produce a working server

// credit to https://github.com/PanGan21/chat/blob/master/server/src/main.rs

use std::{
    collections::HashMap,
    io,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex, Condvar},
    thread,
    str::FromStr,
};

use crate::{
    server_utils::ServerState,
    server_communication,
    server_registration::ServerRegistrationHandler,
    server_lobby::ServerLobbyHandler,
    server_games::ServerGamesHandler,
};

use poker_model::{
    game_types::{
        dealer_game::DealerGameTypes,
        five_card_draw,
        texas_holdem,
    },
    poker_deck::*,
    hand_evaluator::HandEvaluator,
};

use database_handler::DatabaseHandler;

const DB_PATH_FROM_ROOT: &str = "poker_players.db";

/// The Server struct. Must be initialized with a valid IP address for clients to connect to it
pub struct Server {
    address: String,
    // Store clients name + [id, TcpStream, which thread to be in]
    clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
    // store clients name + their game vote
    clients_votes: Arc<Mutex<HashMap<String, DealerGameTypes>>>,
    // cache number of players in the lobby so we know how many have yet to vote for a game
    num_players_lobby: Arc<Mutex<usize>>,
    // database handler to the server (shared to avoid race conditions)
    db: Arc<Mutex<DatabaseHandler>>,
}

impl Server {
    /// create new server
    pub fn new(address: &str) -> Self {
        let db = DatabaseHandler::new(DB_PATH_FROM_ROOT).expect("Failed to initialize database");
        Self {
            address: address.to_string(),
            clients: Arc::new(Mutex::new(HashMap::new())),
            clients_votes: Arc::new(Mutex::new(HashMap::new())),
            num_players_lobby: Arc::new(Mutex::new(0)),
            db: Arc::new(Mutex::new(db)),
        }
    }

    /// function to have server begin hosting
    pub fn run(&self) {
        // create a TCP listener
        let server = TcpListener::bind(&self.address).expect("Listener failed to bind");
        server
            .set_nonblocking(true)
            .expect("Failed to set non-blocking mode");

        // create pointer to server for use in threads
        // NOTE: can we move ownership into ServerRegistrationHandler (not needed here AFAIK)
        let mut server_pointer = Arc::new(Mutex::new(server));

        // main thread stores handles
        let mut main_thread_handles = Vec::new();

        // clone pointers to be passed into client registration
        let reg_clients_ptr = Arc::clone(&self.clients);
        let reg_db_ptr = Arc::clone(&self.db);

        // thread that handles client registration
        let thread_server_client_registration = thread::spawn(move || {
            ServerRegistrationHandler::handle_client_connections(server_pointer, reg_clients_ptr, reg_db_ptr);
        });

        // clone pointers to be passed into client lobby
        let lob_clients_ptr = Arc::clone(&self.clients);
        let lob_clients_votes_ptr = Arc::clone(&self.clients_votes);
        let lob_num_players_ptr = Arc::clone(&self.num_players_lobby);
        let lob_db_ptr = Arc::clone(&self.db);

        // thread that handles client lobby
        let thread_server_client_lobby = thread::spawn(move || {
            ServerLobbyHandler::handle_client_lobbies(lob_clients_ptr, lob_clients_votes_ptr, lob_num_players_ptr, lob_db_ptr);
        });

        // clone pointers to be passed into client lobby
        let game_clients_ptr = Arc::clone(&self.clients);
        let game_clients_votes_ptr = Arc::clone(&self.clients_votes);
        let game_num_players_ptr = Arc::clone(&self.num_players_lobby);
        let game_db_ptr = Arc::clone(&self.db);

        // thread that handles client games
        let thread_server_client_games = thread::spawn(move || {
            ServerGamesHandler::handle_client_games(game_clients_ptr, game_clients_votes_ptr, game_num_players_ptr, game_db_ptr);
        });

        // push thread handles
        main_thread_handles.push(thread_server_client_registration);
        main_thread_handles.push(thread_server_client_lobby);
        main_thread_handles.push(thread_server_client_games);


        // make main thread wait
        for thread in main_thread_handles.drain(..) {
            thread.join().unwrap();
        }
        // clear the thread handle
        main_thread_handles.clear();
    }
}
