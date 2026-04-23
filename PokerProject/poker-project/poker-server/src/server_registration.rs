/// portion of server that handles client registration

use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    server_utils::ServerState,
    server_communication,
};

use database_handler::DatabaseHandler;

/// This struct provides the functionality for handling a client wishes to connect/register
pub struct ServerRegistrationHandler;

impl ServerRegistrationHandler {
    /// Handles client connections.
    pub fn handle_client_connections(
        server: Arc<Mutex<TcpListener>>,
        clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        db: Arc<Mutex<DatabaseHandler>>,
    ) {
        // client registration thread stores handles
        let mut client_registration_thread_handles = Vec::new();

        {
            let server_locked = server.lock().unwrap();
            // runs indefinitely. Whenever a client connects, this runs
            for (id, stream) in server_locked.incoming().enumerate() {
                match stream {
                    Ok(mut socket) => {
                        // clone pointers so that each created thread has its own pointer
                        let mut socket_clone = socket.try_clone().unwrap();
                        let clients_clone = Arc::clone(&clients);
                        let db_clone = Arc::clone(&db);
    
                        // spawn a thread for each connection
                        // "move" moves ownership of all variables called
                        // inside the thread to those names
                        let thread_single_client_registering = thread::spawn(move || {
                            // --- register client ---
                            Self::register_client(
                                &mut socket_clone,
                                clients_clone,
                                id,
                                db_clone,
                            );
                        });
    
                        // store Job Handler
                        client_registration_thread_handles.push(thread_single_client_registering);
                    }
    
                    Err(_) => {}
                }
            }
        }

        // manager thread waits
        for thread in client_registration_thread_handles.drain(..) {
            thread.join().unwrap();
        }
        // clear the thread handles
        client_registration_thread_handles.clear();
    }

    /// Registers a client in the database, and returns their name
    ///
    /// Parameters:
    /// socket - TcpStream: TcpStream socket for client
    /// socket_id - num: id number for the socket
    /// db_handler - ServerDatabaseHandler: used for database access
    ///
    /// Returns:
    /// name - String: Name of client (unique in database)
    pub fn register_client(
        socket: &mut TcpStream,
        connected_clients: Arc<Mutex<HashMap<String, (usize, TcpStream, ServerState)>>>,
        client_id: usize,
        db: Arc<Mutex<DatabaseHandler>>,
    ) {
        // Prompt message
        let message: &str = "Please enter your name: ";
    
        // Loop until a valid (not already connected) name is received
        let mut proposed_client_name: String;
        loop {
            server_communication::write_message(socket, message).expect("Failed to write login prompt");
    
            // Wait for response from client
            proposed_client_name = server_communication::read_message_wait(socket);
            println!("[DEBUG] Server received name: {}", proposed_client_name);
    
            // Check if already connected
            let mut connected_clients_locked = connected_clients.lock().unwrap();
            if !connected_clients_locked.contains_key(&proposed_client_name) {
                break;
            }
    
            let client_exists_msg = format!("{} already connected. Choose another name.", proposed_client_name);
            server_communication::write_message(socket, &client_exists_msg).expect("Failed to write name taken message");
            println!("[DEBUG] Name '{}' already connected", proposed_client_name);
        }
    
        // Add to connected clients
        {
            let socket_clone = socket.try_clone().expect("Failed to clone socket");
            connected_clients
                .lock()
                .unwrap()
                .insert(proposed_client_name.clone(), (client_id, socket_clone, ServerState::Connection));
            println!("[DEBUG] Added {} to connected_clients", proposed_client_name);
        }
    
        // Check/create player profile in DB
        {
            let mut db_locked = db.lock().unwrap();
    
            if db_locked.player_exists(&proposed_client_name) {
                println!("[DEBUG] Player {} exists in database.", proposed_client_name);
            } else {
                db_locked
                    .save_player(&proposed_client_name, 0, 0)
                    .expect("Failed to save new player");
                println!("[DEBUG] Created new player {} in database", proposed_client_name);
            }
        }
    
        // Send welcome message
        let welcome_message = format!("Welcome {}!", proposed_client_name);
        server_communication::write_message(socket, &welcome_message).expect("Failed to send welcome message");
        println!("[DEBUG] Sent welcome message to {}", proposed_client_name);
    }
}
