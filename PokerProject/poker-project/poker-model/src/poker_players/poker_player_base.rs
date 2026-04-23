/// contains base code that ALL PokerPlayer variants should use

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use crate::poker_deck::{Card};

/// Determines what the player's last action/current status is
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerStatus {
    Active,     // The player is still in the current hand and actively participating
    Checked,    // The player has checked
    Called,     // The player has called
    Raised,     // The player has raised
    Folded,     // The player has folded and is no longer participating in the current hand
    AllIn,      // Special case: the player is all in (from paying ante, calling, raising)
    Eliminated, // Speical case: the player could not gain chips after going all-in, and is eliminated
}

/// trait to enforce commonality between PokerPlayer-specific structs
pub trait PokerPlayer {
    fn get_name(&self) -> String;
    fn receive_chips(&mut self, num_chips: usize);
    fn get_num_chips(&self) -> usize;
    fn get_status(&self) -> PlayerStatus;
    fn set_status(&mut self, status: PlayerStatus);
    fn get_hand(&self) -> Vec<Card>;
    fn clear_hand(&mut self);
    fn get_contributed_chips(&self) -> usize;
    fn set_contributed_chips(&mut self, amount: usize);
    fn pay(&mut self, num: usize);
}

/// base struct with predefined functions that other PokerPlayer-structs will use
#[derive(Debug, Clone)]
pub struct PokerPlayerBase {
    pub name: String,
    pub chips: usize,
    pub status_in_game: PlayerStatus,
    pub contributed_chips: usize,
}

// basic functions (to be wrapped)
impl PokerPlayerBase {
    /// create a new PokerPlayerBase
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            chips: 10,    // default no. of chips players begin with
            status_in_game: PlayerStatus::Active,
            contributed_chips: 0,
        }
    }

    /// get the name of the player
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// add chips to the player
    pub fn receive_chips(&mut self, num_chips: usize) {
        self.chips += num_chips;
    }

    /// get the number of chips the player has
    pub fn get_num_chips(&self) -> usize {
        self.chips.clone()
    }

    /// get the player's PlayerStatus status
    pub fn get_status(&self) -> PlayerStatus {
        self.status_in_game.clone()
    }

    /// set the player's PlayerStatus status
    pub fn set_status(&mut self, status: PlayerStatus) {
        self.status_in_game = status;
    }

    /// get the number of chips the player contributed for the specific betting round
    pub fn get_contributed_chips(&self) -> usize {
        self.contributed_chips
    }
    
    /// set the number of chips the player contributed for the specific betting round
    pub fn set_contributed_chips(&mut self, amount: usize) {
        self.contributed_chips = amount;
    }

    /// the player pays (loses) the num. If they have less chips than num, they pay everything.
    /// This function does not account who the player pays to, only deducting from player
    pub fn pay(&mut self, num: usize) {
        self.chips = self.chips.saturating_sub(num);
        self.set_contributed_chips(self.get_contributed_chips() + num);
    }
}
