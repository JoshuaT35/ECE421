/// contains base code that ALL DealerGame variants should use

use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString, EnumString};

use crate::poker_deck::{Deck};
use crate::poker_deck::Card;

/// Enum for the possible game types
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, ToString, EnumString)] // Add EnumIter to enable iter()
pub enum DealerGameTypes {
    FiveCardDraw,
    TexasHoldem,
    SevenCardStud,
}

/// trait to enforce commonality between DealerGame-specific structs
pub trait DealerGame {
    fn deal_cards(&mut self, num_to_draw: usize) -> Vec<Card>;
    fn shuffle_deck(&mut self);
    fn reset_deck(&mut self);
    fn get_ante(&self) -> usize;
    fn get_current_bet(&self) -> usize;
    fn set_current_bet(&mut self, amount: usize);
    fn get_pot(&self) -> usize;
    fn add_to_pot(&mut self, amount: usize);
    fn clear_pot(&mut self);
    fn get_current_round(&self) -> u8;
    fn next_round(&mut self);
}

// basic struct for a poker game. All poker games act as a wrapper function for this
pub struct PokerGameBase {
    pub deck: Deck,
    pub pot: usize,
    pub ante: usize,
    pub current_round: u8,
    pub current_bet: usize,
}

// basic functions (to be wrapped)
impl PokerGameBase {
    /// create a new PokerGameBase
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            pot: 0,
            ante: 1,
            current_round: 1,
            current_bet: 0,
        }
    }

    /// deal cards from the deck
    pub fn deal_cards(&mut self, num_to_draw: usize) -> Vec<Card> {
        self.deck.draw(num_to_draw)
    }

    /// shuffle the deck
    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    /// reset the deck to 52 ordered cards
    pub fn reset_deck(&mut self) {
        self.deck = Deck::new();
    }

    /// get the ante for the round
    pub fn get_ante(&self) -> usize {
        self.ante
    }

    /// get the current bet for the betting round
    pub fn get_current_bet(&self) -> usize {
        self.current_bet
    }
    
    /// set the current bet for the betting round
    pub fn set_current_bet(&mut self, amount: usize) {
        self.current_bet = amount;
    }    

    /// get the value of the pot
    pub fn get_pot(&self) -> usize {
        self.pot
    }

    /// add to the pot
    pub fn add_to_pot(&mut self, amount: usize) {
        self.pot += amount;
    }

    /// empty the pot
    pub fn clear_pot(&mut self) {
        self.pot = 0;
    }

    /// get the current round
    pub fn get_current_round(&self) -> u8 {
        self.current_round
    }

    /// incremenet to the next current round
    pub fn next_round(&mut self) {
        self.current_round += 1;
    }
}
