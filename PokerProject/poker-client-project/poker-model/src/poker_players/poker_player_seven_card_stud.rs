/// poker model for a player in seven card stud

use crate::{
    poker_deck::Card,
    poker_players::poker_player_base::*,
};

/// struct for a PokerPlayer in a Seven Card Stud game
pub struct PokerPlayerSevenCardStud {
    base: PokerPlayerBase,
    face_down_hand: Vec<Card>,
    pub face_up_hand: Vec<Card>,
}

/// unique functions for PokerPlayerSevenCardStud
impl PokerPlayerSevenCardStud {
    /// create a new PokerPlayerSevenCardStud
    pub fn new(name: &str) -> Self {
        Self {
            base: PokerPlayerBase::new(name),
            face_down_hand: Vec::new(),
            face_up_hand: Vec::new(),
        }
    }

    /// get the player's face down cards
    pub fn get_face_down_cards(&self) -> Vec<Card> {
        self.face_down_hand.clone()
    }

    /// get the player's face up cards
    pub fn get_face_up_cards(&self) -> Vec<Card> {
        self.face_up_hand.clone()
    }

    /// get the number of face down cards the players has
    pub fn get_num_face_down_cards(&self) -> usize {
        self.face_down_hand.len()
    }

    /// get the number of face up cards the players has
    pub fn get_num_face_up_cards(&self) -> usize {
        self.face_up_hand.len()
    }

    /// add a card to the player's face down cards
    pub fn receive_face_down_cards(&mut self, cards: Vec<Card>) {
        self.face_down_hand.extend(cards);
    }

    /// add a card to the player's face up cards
    pub fn receive_face_up_cards(&mut self, cards: Vec<Card>) {
        self.face_up_hand.extend(cards);
    }
}

// PokerPlayerSevenCardStud defines ALL functions declared in PokerPlayer, using PokerPlayerBase
impl PokerPlayer for PokerPlayerSevenCardStud {
    /// get the name of the player
    fn get_name(&self) -> String {
        self.base.get_name()
    }

    /// add chips to the player
    fn receive_chips(&mut self, num_chips: usize) {
        self.base.receive_chips(num_chips);
    }

    /// get the number of chips the player has
    fn get_num_chips(&self) -> usize {
        self.base.get_num_chips()
    }

    /// get the player's PlayerStatus status
    fn get_status(&self) -> PlayerStatus {
        self.base.get_status()
    }

    /// set the player's PlayerStatus status
    fn set_status(&mut self, status: PlayerStatus) {
        self.base.set_status(status);
    }

    /// get the player's hand
    fn get_hand(&self) -> Vec<Card> {
        let mut hand = self.face_down_hand.clone();
        hand.extend(self.face_up_hand.clone());
        hand
    }

    /// clear the player's hand
    fn clear_hand(&mut self) {
        self.face_down_hand.clear();
        self.face_up_hand.clear();
    }

    /// get the number of chips the player contributed for the specific betting round
    fn get_contributed_chips(&self) -> usize {
        self.base.get_contributed_chips()
    }

    /// set the number of chips the player contributed for the specific betting round
    fn set_contributed_chips(&mut self, amount: usize) {
        self.base.set_contributed_chips(amount);
    }

    /// the player pays (loses) the num. If they have less chips than num, they pay everything.
    /// This function does not account who the player pays to, only deducting from player
    fn pay(&mut self, num: usize) {
        self.base.pay(num)
    }
}
