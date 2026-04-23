/// poker model for a player in texas hold'em

use itertools::Itertools;
use std::cmp::Ordering;

use crate::{
    poker_deck::Card,
    poker_players::poker_player_base::*,
    hand_evaluator::{HandEvaluator, HandRank},
};

/// struct for a PokerPlayer in a Texas Hold'em game
pub struct PokerPlayerTexasHoldem {
    base: PokerPlayerBase,
    pub hand: Vec<Card>,
}

/// unique functions for PokerPlayerTexasHoldem
impl PokerPlayerTexasHoldem {
    /// create a new PokerPlayerTexasHoldem
    pub fn new(name: &str) -> Self {
        Self {
            base: PokerPlayerBase::new(name),
            hand: Vec::new(),
        }
    }

    /// get the number of cards the players has
    pub fn get_num_cards(&self) -> usize {
        self.hand.len()
    }

    /// add card(s) to the player's hand
    pub fn receive_cards(&mut self, cards: Vec<Card>) {
        self.hand.extend(cards);
    }

    /// get the player's hand + community cards
    pub fn get_hand_with_community(&self, community: &Vec<Card>) -> Vec<Card> {
        let mut combined = self.hand.clone();
        combined.extend_from_slice(community);
        combined
    }

    /// get the best 5-card hand with the player's hand + community cards
    pub fn best_hand_with_community(&self, community_cards: &Vec<Card>) -> Vec<Card> {
        let mut all_cards = self.hand.clone();
        all_cards.extend_from_slice(community_cards);
    
        let mut best_rank: Option<HandRank> = None;
        let mut best_hand = vec![];
    
        for combo in all_cards.iter().copied().combinations(5) {
            let rank = HandEvaluator::evaluate_hand(&combo);
            if best_rank.is_none() || rank.compare(best_rank.as_ref().unwrap()) == Ordering::Greater {
                best_rank = Some(rank);
                best_hand = combo;
            }
        }

    best_hand
    }
    
}

// PokerPlayerTexasHoldem defines ALL functions declared in PokerPlayer, using PokerPlayerBase
impl PokerPlayer for PokerPlayerTexasHoldem {
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
        self.hand.clone()
    }

    /// clear the player's hand
    fn clear_hand(&mut self) {
        self.hand.clear();
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
