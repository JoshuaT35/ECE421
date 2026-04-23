use poker_model::*;
use poker_model::poker_deck::{Card, Rank, Suit, Deck};
use poker_model::hand_evaluator::HandRank;
use std::cmp::Ordering;

#[cfg(test)]
mod evaluation_tests {
    use super::*;

    #[test]
    fn royal_flush() {
        let mut deck = Deck::new();
        let hand = deck.draw(5);
        assert_eq!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::RoyalFlush);
    }

    #[test]
    fn straight_flush() {
        let mut deck = Deck::new();
        let _ = deck.draw(1);
        let hand = deck.draw(5);
        assert_eq!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::StraightFlush(Rank::King));
    }

    #[test]
    fn four_of_a_kind() {
        let mut deck = Deck::new();
        let mut hand = Vec::new();
        for _ in 0..3 {
            hand.extend(deck.draw(1));
            let _ = deck.draw(12);
        }
        hand.extend(deck.draw(2));
        assert_eq!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::FourOfAKind(Rank::A, Rank::King));
    }

    #[test]
    fn full_house() {
        let mut deck = Deck::new();
        let mut hand = Vec::new();
        for _ in 0..2 {
            hand.extend(deck.draw(2));
            let _ = deck.draw(11);
        }
        hand.extend(deck.draw(1));
        assert_eq!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::FullHouse(Rank::A, Rank::King));
    }

    #[test]
    fn all_flushes() {
        let suits = [
            (0, Suit::Spades),
            (13, Suit::Clubs),
            (26, Suit::Diamonds),
            (39, Suit::Hearts),
        ];

        for (skip, suit) in suits {
            let mut deck = Deck::new();
            let _ = deck.draw(skip);
            let mut hand = Vec::new();
            for _ in 0..5 {
                hand.extend(deck.draw(1));
                let _ = deck.draw(1);
            }
            let ranks = vec![Rank::A, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Six];
            assert_eq!(
                hand_evaluator::HandEvaluator::evaluate_hand(&hand),
                HandRank::Flush(ranks.clone(), suit)
            );
        }
    }

    #[test]
    fn mismatch_flush() {
        let mut deck = Deck::new();
        let mut hand1 = Vec::new();
        let mut hand2 = Vec::new();
        hand1.extend(deck.draw(4));
        let _ = deck.draw(1);
        hand1.extend(deck.draw(1));
        let _ = deck.draw(7);
        hand2.extend(deck.draw(4));
        let _ = deck.draw(1);
        hand2.extend(deck.draw(1));
        assert_ne!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand1),
            hand_evaluator::HandEvaluator::evaluate_hand(&hand2)
        );
    }

    #[test]
    fn near_flush_not_flush() {
        let mut deck = Deck::new();
        let mut hand = Vec::new();
        for _ in 0..4 {
            hand.extend(deck.draw(1));
            let _ = deck.draw(1);
        }
        let _ = deck.draw(13);
        hand.extend(deck.draw(1));
        let ranks = vec![Rank::A, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Six];
        assert_ne!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand),
            HandRank::Flush(ranks, Suit::Spades)
        );
    }

    #[test]
    fn straight() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(1);
        let _ = deck.draw(13);
        hand.extend(deck.draw(4));
        assert_eq!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::Straight(Rank::A));
    }

    #[test]
    fn fake_straight() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(2);
        let _ = deck.draw(8);
        hand.extend(deck.draw(3));
        assert_ne!(hand_evaluator::HandEvaluator::evaluate_hand(&hand), HandRank::Straight(Rank::Four));
    }

    #[test]
    fn three_of_a_kind() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(3);
        let _ = deck.draw(10);
        hand.extend(deck.draw(1));
        let _ = deck.draw(12);
        hand.extend(deck.draw(1));
        assert_eq!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand),
            HandRank::ThreeOfAKind(Rank::A, vec![Rank::King, Rank::Queen])
        );
    }

    #[test]
    fn two_pair() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(3);
        let _ = deck.draw(10);
        hand.extend(deck.draw(2));
        assert_eq!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand),
            HandRank::TwoPair(Rank::A, Rank::King, Rank::Queen)
        );
    }

    #[test]
    fn one_pair() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(1);
        let _ = deck.draw(12);
        hand.extend(deck.draw(4));
        assert_eq!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand),
            HandRank::OnePair(Rank::A, vec![Rank::King, Rank::Queen, Rank::Jack])
        );
    }

    #[test]
    fn high_card() {
        let mut deck = Deck::new();
        let mut hand = deck.draw(1);
        let _ = deck.draw(14);
        hand.extend(deck.draw(4));
        assert_eq!(
            hand_evaluator::HandEvaluator::evaluate_hand(&hand),
            HandRank::HighCard(vec![Rank::A, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine])
        );
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::*;

    #[test]
    fn compare_rflush_sflush() {
        assert_eq!(HandRank::RoyalFlush.compare(&HandRank::StraightFlush(Rank::King)), Ordering::Greater);
    }

    #[test]
    fn compare_straight_flushes() {
        assert_eq!(HandRank::StraightFlush(Rank::King).compare(&HandRank::StraightFlush(Rank::Queen)), Ordering::Greater);
    }

    #[test]
    fn straight_flush_beats_four_kind() {
        assert_eq!(
            HandRank::StraightFlush(Rank::Five).compare(&HandRank::FourOfAKind(Rank::A, Rank::King)),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_four_kinds() {
        assert_eq!(
            HandRank::FourOfAKind(Rank::King, Rank::Three).compare(&HandRank::FourOfAKind(Rank::Two, Rank::A)),
            Ordering::Greater
        );
    }

    #[test]
    fn four_kind_vs_full_house() {
        assert_eq!(
            HandRank::FourOfAKind(Rank::Two, Rank::Three).compare(&HandRank::FullHouse(Rank::A, Rank::King)),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_full_houses() {
        assert_eq!(
            HandRank::FourOfAKind(Rank::Two, Rank::A).compare(&HandRank::FourOfAKind(Rank::Three, Rank::Four)),
            Ordering::Less
        );
    }

    #[test]
    fn full_house_vs_flush() {
        assert_eq!(
            HandRank::FullHouse(Rank::Two, Rank::Three).compare(&HandRank::Flush(
                vec![Rank::A, Rank::Queen, Rank::Five, Rank::Four, Rank::Three],
                Suit::Spades
            )),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_flushes() {
        assert_eq!(
            HandRank::Flush(vec![Rank::A, Rank::Queen, Rank::Five, Rank::Four, Rank::Three], Suit::Spades)
                .compare(&HandRank::Flush(vec![Rank::King, Rank::Queen, Rank::Ten, Rank::Nine, Rank::Eight], Suit::Clubs)),
            Ordering::Greater
        );
    }

    #[test]
    fn flush_vs_straight() {
        assert_eq!(
            HandRank::Flush(vec![Rank::Seven, Rank::Six, Rank::Four, Rank::Three, Rank::Two], Suit::Spades)
                .compare(&HandRank::Straight(Rank::A)),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_straights() {
        assert_eq!(HandRank::Straight(Rank::A).compare(&HandRank::Straight(Rank::King)), Ordering::Greater);
    }

    #[test]
    fn straight_vs_three_kind() {
        assert_eq!(
            HandRank::Straight(Rank::Five).compare(&HandRank::ThreeOfAKind(Rank::A, vec![Rank::King, Rank::Queen])),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_three_kinds() {
        assert_eq!(
            HandRank::ThreeOfAKind(Rank::Ten, vec![Rank::Jack, Rank::Nine])
                .compare(&HandRank::ThreeOfAKind(Rank::Nine, vec![Rank::A, Rank::King])),
            Ordering::Greater
        );
    }

    #[test]
    fn three_kind_vs_two_pair() {
        assert_eq!(
            HandRank::ThreeOfAKind(Rank::Two, vec![Rank::Three, Rank::Four])
                .compare(&HandRank::TwoPair(Rank::A, Rank::King, Rank::Queen)),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_two_pairs() {
        assert_eq!(
            HandRank::TwoPair(Rank::King, Rank::Queen, Rank::Ten)
                .compare(&HandRank::TwoPair(Rank::Queen, Rank::Jack, Rank::A)),
            Ordering::Greater
        );
    }

    #[test]
    fn two_pair_vs_one_pair() {
        assert_eq!(
            HandRank::TwoPair(Rank::Three, Rank::Two, Rank::Four)
                .compare(&HandRank::OnePair(Rank::A, vec![Rank::King, Rank::Queen, Rank::Jack])),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_one_pairs() {
        assert_eq!(
            HandRank::OnePair(Rank::A, vec![Rank::King, Rank::Queen, Rank::Jack])
                .compare(&HandRank::OnePair(Rank::Jack, vec![Rank::A, Rank::King, Rank::Queen])),
            Ordering::Greater
        );
    }

    #[test]
    fn one_pair_vs_high_card() {
        assert_eq!(
            HandRank::OnePair(Rank::Two, vec![Rank::Three, Rank::Four, Rank::Five])
                .compare(&HandRank::HighCard(vec![Rank::A, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine])),
            Ordering::Greater
        );
    }

    #[test]
    fn compare_high_cards() {
        assert_eq!(
            HandRank::HighCard(vec![Rank::King, Rank::Queen, Rank::Jack, Rank::Nine, Rank::Eight])
                .compare(&HandRank::HighCard(vec![Rank::A, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine])),
            Ordering::Less
        );
    }
}
