//! # Poker Server Module
//!
//! This crate provides all server-side logic required to host and manage a multiplayer poker game.
//! It includes functionality for player registration, lobby management, game execution, and communication over TCP.
//!
//! The server ties together components from other crates (like the game engine, players, and deck logic)
//! to simulate a real-world poker environment.
//!
//! ## Modules
//!
//! - [`server_registration`] – Handles incoming client connections and player registration
//! - [`server_lobby`] – Manages the game lobby, including player readiness and game start conditions
//! - [`server_games`] – Orchestrates different game types (e.g., Five Card Draw, Seven Card Stud)
//! - [`server_communication`] – Provides message formatting and TCP I/O with connected clients
//! - [`server_utils`] – Helper functions and shared utilities used throughout the server logic
//! - [`game_logic`] – Game rules and state management (delegates to core game modules)
//!
//! ## Example
//!
//! This crate is typically used by launching a TCP server that listens for players and runs poker matches.
//! To start a server:
//!
//! ```rust ignore 
//! use poker_server::server_registration::ServerRegistrationHandler;
//!
//! ServerRegistrationHandler::handle_client_connections();
//! ```
//!
//! This server interacts with clients through messages and coordinates gameplay using `server_games`.
//!
//! ## Related Crates
//!
//! - `poker_model` – Models used for games.
//! - `poker_client` – Client-side code.
//! - `database_handler` – Persistent storage for player stats and history.

pub mod server_utils;
pub mod server_communication;
pub mod game_logic;
pub mod server_registration;
pub mod server_lobby;
pub mod server_games;
