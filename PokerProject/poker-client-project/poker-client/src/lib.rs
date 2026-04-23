//! # Poker Client Module
//!
//! This crate provides all client-side logic required to host and manage a multiplayer poker game.
//! It includes functionality for the GUI and communication over TCP.
//!
//! ## Modules
//!
//! - [`hand_evaluator`] – Hand ranking logic for all poker hands
//! - [`poker_deck`] – Card deck generation, shuffling, and dealing
//! - [`poker_players`] – Player traits and player implementations
//! - [`game_types`] – Game-specific rules and logic (e.g., Five Card Draw)
//!
//! ## Related Crates
//!
//! - `poker_client` – Client-side code.
//! - `poker_server` - Server-side code.
//! - `database_handler` – Persistent storage for player stats and history.

pub mod client_communication;
pub mod client_app;
