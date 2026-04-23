use std::net::TcpStream;
use std::thread;

use poker_model::game_types::dealer_game::DealerGame;
use poker_model::game_types::seven_card_stud::SevenCardStudDealer;
use poker_model::poker_players::poker_player_base::{PlayerStatus, PokerPlayer};
use poker_model::poker_players::poker_player_seven_card_stud::PokerPlayerSevenCardStud;
use poker_model::poker_deck::{Card, Rank, Suit, create_card_message};
use poker_model::hand_evaluator::{HandEvaluator, HandRank};

#[test]
// Tests face-up and face-down card separation
fn test_face_up_down_card_separation() {
    let mut player = PokerPlayerSevenCardStud::new("TestPlayer");
    let mut dealer = SevenCardStudDealer::new();
    
    // Add face-down cards
    player.receive_face_down_cards(dealer.deal_cards(2));
    assert_eq!(player.get_num_face_down_cards(), 2);
    assert_eq!(player.get_num_face_up_cards(), 0);
    
    // Add face-up cards
    player.receive_face_up_cards(dealer.deal_cards(1));
    assert_eq!(player.get_num_face_down_cards(), 2);
    assert_eq!(player.get_num_face_up_cards(), 1);
    
    // Total hand should contain both
    assert_eq!(player.get_hand().len(), 3);
}

#[test]
// Tests stud game betting capabilities
fn test_stud_betting_rounds() {
    let mut dealer = SevenCardStudDealer::new();
    
    // Initial state
    assert_eq!(dealer.get_current_round(), 1);
    assert_eq!(dealer.get_current_bet(), 0);
    
    // Set a bet
    dealer.set_current_bet(5);
    assert_eq!(dealer.get_current_bet(), 5);
    
    // Advance to next round
    dealer.next_round();
    assert_eq!(dealer.get_current_round(), 2);
    
    // In a new round, bet might be cleared (depends on implementation)
    dealer.set_current_bet(10);
    assert_eq!(dealer.get_current_bet(), 10);
    
    // Check pot functionality
    dealer.add_to_pot(10);
    assert_eq!(dealer.get_pot(), 10);
    
    dealer.clear_pot();
    assert_eq!(dealer.get_pot(), 0);
}

#[test]
// Tests the evaluation of the best 5-card hand from 7 cards
fn test_best_five_card_selection() {
    let mut player = PokerPlayerSevenCardStud::new("TestPlayer");
    
    // Create a specific hand with both strong and weak cards
    player.receive_face_down_cards(vec![
        Card { rank: Rank::A, suit: Suit::Spades },
        Card { rank: Rank::A, suit: Suit::Hearts },
        Card { rank: Rank::Two, suit: Suit::Clubs },
    ]);
    
    player.receive_face_up_cards(vec![
        Card { rank: Rank::A, suit: Suit::Diamonds },
        Card { rank: Rank::King, suit: Suit::Spades },
        Card { rank: Rank::Queen, suit: Suit::Spades },
        Card { rank: Rank::Three, suit: Suit::Diamonds },
    ]);
    
    // Evaluate best 5-card hand
    let best_hand = HandEvaluator::best_five_hand_from_larger(player.get_hand());
    
    // The best hand should contain the three Aces
    let ace_count = best_hand.iter()
        .filter(|card| card.rank == Rank::A)
        .count();
    
    assert_eq!(best_hand.len(), 5);
    assert_eq!(ace_count, 3);
    
    // Evaluate the hand type
    let rank = HandEvaluator::evaluate_hand(&best_hand);
    
    // Should be three of a kind or better
    match rank {
        HandRank::ThreeOfAKind(_, _) |
        HandRank::FullHouse(_, _) |
        HandRank::FourOfAKind(_, _) |
        HandRank::StraightFlush(_) |
        HandRank::RoyalFlush => assert!(true),
        _ => assert!(false, "Expected three of a kind or better, got {:?}", rank),
    }
}