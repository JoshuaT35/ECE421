/// poker model for seven card stud

use crate::{
    poker_deck::{Deck, Card},
    game_types::dealer_game::{
        DealerGame,
        PokerGameBase,
    },
};

// struct for seven card stud game
pub struct SevenCardStudDealer {
    base: PokerGameBase,
}

// unique functions
impl SevenCardStudDealer {
    /// create a new SevenCardStudDealer
    pub fn new() -> Self {
        Self {
            base: PokerGameBase::new(),
        }
    }

    /// initial setup environment for the game
    pub fn setup(&mut self) {
        self.base.shuffle_deck();
    }
}

// SevenCardStudDealer defines functions declared in DealerGame, using PokerGameBase
impl DealerGame for SevenCardStudDealer {
    /// deal cards from the deck
    fn deal_cards(&mut self, num_to_draw: usize) -> Vec<Card> {
        self.base.deal_cards(num_to_draw)
    }

    /// shuffle the deck
    fn shuffle_deck(&mut self) {
        self.base.shuffle_deck();
    }

    /// reset the deck to 52 ordered cards
    fn reset_deck(&mut self) {
        self.base.reset_deck();
    }

    /// get the ante for the round
    fn get_ante(&self) -> usize {
        self.base.get_ante()
    }

    /// get the current bet for the betting round
    fn get_current_bet(&self) -> usize {
        self.base.current_bet
    }
    
    /// set the current bet for the betting round
    fn set_current_bet(&mut self, amount: usize) {
        self.base.current_bet = amount;
    }
    
    /// get the value of the pot
    fn get_pot(&self) -> usize {
        self.base.get_pot()
    }

    /// add to the pot
    fn add_to_pot(&mut self, amount: usize) {
        self.base.add_to_pot(amount);
    }

    /// empty the pot
    fn clear_pot(&mut self) {
        self.base.clear_pot();
    }

    /// get the current round
    fn get_current_round(&self) -> u8 {
        self.base.get_current_round()
    }

    /// incremenet to the next current round
    fn next_round(&mut self) {
        self.base.next_round();
    }
}
