/// poker model for texas hold em

use crate::{
    poker_deck::{Deck, Card},
    game_types::dealer_game::{
        DealerGame,
        PokerGameBase,
    },
};

/// Texas Hold'em game implementation.
/// In Texas Hold'em, each player receives 2 hole cards.
/// Then the dealer deals community cards in phases:
/// - The Flop: 3 community cards
/// - The Turn: 1 community card
/// - The River: 1 community card

// struct for five card draw game
pub struct TexasHoldemDealer {
    base: PokerGameBase,
    community_cards: Vec<Card>,
}

// unique functions
impl TexasHoldemDealer {
    /// create a new TexasHoldemDealer
    pub fn new() -> Self {
        Self {
            base: PokerGameBase::new(),
            community_cards: Vec::new(),
        }
    }

    /// initial setup environment for the game
    pub fn setup(&mut self) {
        self.base.shuffle_deck();
        self.community_cards.clear();
    }


    /// add cards to the community cards
    pub fn deal_community_cards(&mut self, count: usize) {
        let cards = self.base.deck.draw(count);
        self.community_cards.extend(cards);
    }

    /// get the current community cards
    pub fn get_community_cards(&self) -> &Vec<Card> {
        &self.community_cards
    }

    /// clear all community cards
    pub fn clear_community_cards(&mut self) {
        self.community_cards.clear();
    }

    /// Deals 2 hole cards to a player.
    pub fn deal_hole_cards(&mut self) -> Vec<Card> {
        self.base.deck.draw(2)
    }

    /// Deals the flop: 3 community cards.
    pub fn deal_flop(&mut self) -> Vec<Card> {
        self.base.deck.draw(3)
    }

    /// Deals the turn: 1 community card.
    pub fn deal_turn(&mut self) -> Option<Card> {
        self.base.deck.draw(1).pop()
    }

    /// Deals the river: 1 community card.
    pub fn deal_river(&mut self) -> Option<Card> {
        self.base.deck.draw(1).pop()
    }
}

// TexasHoldemDealer defines functions declared in DealerGame, using PokerGameBase
impl DealerGame for TexasHoldemDealer {
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
