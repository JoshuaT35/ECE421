use poker_model::game_types::dealer_game::*;
use poker_model::game_types::texas_holdem::*;
use poker_model::poker_deck::{Deck, Card, Rank, Suit};

#[cfg(test)]
mod texas_holdem_tests {
    use super::*;

    #[test]
    fn test_new_game_initializes_correctly() {
        let dealer = TexasHoldemDealer::new();
        assert_eq!(dealer.get_community_cards().len(), 0);
        assert_eq!(dealer.get_pot(), 0);
        assert_eq!(dealer.get_current_round(), 1);
    }

    #[test]
    fn test_setup_resets_game_state() {
        let mut dealer = TexasHoldemDealer::new();
        dealer.deal_community_cards(3);

        dealer.setup();

        assert_eq!(dealer.get_community_cards().len(), 0);
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut dealer = TexasHoldemDealer::new();
        dealer.setup();
        let cards = dealer.deal_hole_cards();
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn test_deal_community_cards_accumulates() {
        let mut dealer = TexasHoldemDealer::new();
        dealer.setup();

        dealer.deal_community_cards(2);
        dealer.deal_community_cards(1);
        assert_eq!(dealer.get_community_cards().len(), 3);
    }

    #[test]
    fn test_clear_community_cards() {
        let mut dealer = TexasHoldemDealer::new();
        dealer.setup();

        dealer.deal_community_cards(3);
        assert_eq!(dealer.get_community_cards().len(), 3);

        dealer.clear_community_cards();
        assert_eq!(dealer.get_community_cards().len(), 0);
    }

    #[test]
    fn test_dealer_game_trait_next_round() {
        let mut dealer = TexasHoldemDealer::new();
        let current_round = dealer.get_current_round();
        dealer.next_round();
        assert_eq!(dealer.get_current_round(), current_round + 1);
    }
}
