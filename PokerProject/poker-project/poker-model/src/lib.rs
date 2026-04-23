//! # Poker Model Module
//!
//! This crate provides the essential game logic needed to manage a multiplayer poker game.
//! The Poker Model module is responsible for handling poker-related logic such as player actions, 
//! hand evaluations, game rules, and the overall structure of the game.
//!
//! It is not intended for direct usage but to be used by a server or other components managing the game state.
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

pub mod hand_evaluator;
pub mod poker_deck;
pub mod poker_players;
pub mod game_types;