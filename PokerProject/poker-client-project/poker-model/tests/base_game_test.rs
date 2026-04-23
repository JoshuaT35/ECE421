use poker_model::game_types::dealer_game::*;
use poker_model::poker_deck::{Deck, Card, Rank, Suit};
use strum::IntoEnumIterator; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_defaults() {
        let game = PokerGameBase::new();
        assert_eq!(game.pot, 0);
        assert_eq!(game.ante, 1);
        assert_eq!(game.current_round, 1);
        assert_eq!(game.current_bet, 0);
    }

    #[test]
    fn test_shuffle_and_reset_deck() {
        let mut game = PokerGameBase::new();
        let original_deck = game.deck.cards.clone();
        game.shuffle_deck();

        assert_eq!(game.deck.cards.len(), 52);
        assert_ne!(game.deck.cards, original_deck);     // although this can't be guaranteed to pass, incredibly unlikely to fail

        game.reset_deck();
        assert_eq!(game.deck.cards.len(), 52);
        assert_eq!(game.deck.cards, original_deck); 
    }

    #[test]
    fn test_deal_cards_reduces_deck() {
        let mut game = PokerGameBase::new();
        let original_len = game.deck.cards.len();
        let drawn = game.deal_cards(5);
        assert_eq!(drawn.len(), 5);
        assert_eq!(game.deck.cards.len(), original_len - 5);
    }

    #[test]
    fn test_get_and_set_bet() {
        let mut game = PokerGameBase::new();
        assert_eq!(game.get_current_bet(), 0);
        game.set_current_bet(50);
        assert_eq!(game.get_current_bet(), 50);
    }

    #[test]
    fn test_add_and_clear_pot() {
        let mut game = PokerGameBase::new();
        game.add_to_pot(100);
        assert_eq!(game.get_pot(), 100);
        game.add_to_pot(50);
        assert_eq!(game.get_pot(), 150);
        game.clear_pot();
        assert_eq!(game.get_pot(), 0);
    }

    #[test]
    fn test_round_tracking() {
        let mut game = PokerGameBase::new();
        assert_eq!(game.get_current_round(), 1);
        game.next_round();
        assert_eq!(game.get_current_round(), 2);
        game.next_round();
        assert_eq!(game.get_current_round(), 3);
    }

    #[test]
    fn test_get_ante() {
        let game = PokerGameBase::new();
        assert_eq!(game.get_ante(), 1);
    }

    #[test]
    fn test_enum_conversion() {
        use std::str::FromStr;

        let game_type = DealerGameTypes::from_str("TexasHoldem").unwrap();
        assert_eq!(game_type, DealerGameTypes::TexasHoldem);

        let game_type = DealerGameTypes::from_str("FiveCardDraw").unwrap();
        assert_eq!(game_type, DealerGameTypes::FiveCardDraw);

        let game_type = DealerGameTypes::from_str("SevenCardStud").unwrap();
        assert_eq!(game_type, DealerGameTypes::SevenCardStud);
    }

    #[test]
    fn test_enum_iteration() {
        let all_games: Vec<DealerGameTypes> = DealerGameTypes::iter().collect();
        assert_eq!(all_games.len(), 3);
        assert!(all_games.contains(&DealerGameTypes::FiveCardDraw));
        assert!(all_games.contains(&DealerGameTypes::TexasHoldem));
        assert!(all_games.contains(&DealerGameTypes::SevenCardStud));
    }
}
