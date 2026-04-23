//! # Database Handler Module
//!
//! This crate provides functionality to manage poker player data using SQLite.
//! It is responsible for saving and retrieving player statistics, such as games won and lost.
//!
//! This module is typically used on the server-side to persist data across game sessions.
//!
//! ## Example
//!
//! ```rust
//! use database_handler::DatabaseHandler;
//!
//! let db = DatabaseHandler::new("players.db").expect("Failed to initialize database");
//!
//! db.save_player("Alice", 5, 2).unwrap();
//! let player_data = db.get_player("Alice").unwrap();
//! ```
//!
//! ## Schema
//!
//! The `players_info` table has the following structure:
//!
//! - `name` (TEXT, PRIMARY KEY)
//! - `games_won` (INTEGER)
//! - `games_lost` (INTEGER)
//!
//! ## Related Crates
//!
//! - `poker_model` – Models used for games.
//! - `poker_client` – Client-side code.
//! - `poker_server` - Server-side code.

use rusqlite::{params, Connection, Result};

/// Handles SQLite-based storage and retrieval of player data.
pub struct DatabaseHandler {
    /// Connection to the SQLite database.
    pub conn: Connection,
}

impl DatabaseHandler {
    /// Creates a new database handler, initializing the `players_info` table if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `db_path` - A path to the SQLite database file.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened or initialized.
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS players_info (
                name TEXT PRIMARY KEY,
                games_won INTEGER NOT NULL,
                games_lost INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    /// Inserts a new player or updates an existing player's win/loss stats.
    pub fn save_player(
        &self,
        p_name: &str,
        p_games_won: i64,
        p_games_lost: i64,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO players_info (name, games_won, games_lost)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(name) DO UPDATE SET
                games_won = ?2,
                games_lost = ?3",
            params![p_name, p_games_won, p_games_lost],
        )?;
        Ok(())
    }

    /// Checks if a player already exists in the database.
    pub fn player_exists(&self, name: &str) -> bool {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM players_info WHERE name = ?1").unwrap();
        let count: i64 = stmt.query_row(params![name], |row| row.get(0)).unwrap();
        count > 0
    }

    /// Retrieves a player's win/loss data.
    ///
    /// Returns `Ok(Some((games_won, games_lost)))` if the player exists, `Ok(None)` otherwise.
    pub fn get_player(&self, name: &str) -> Result<Option<(i64, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT games_won, games_lost FROM players_info WHERE name = ?1",
        )?;
    
        let result = stmt.query_row(params![name], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
            ))
        });

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
