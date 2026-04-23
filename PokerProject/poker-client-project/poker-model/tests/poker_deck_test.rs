use poker_model::poker_deck::{Deck, Card, Rank, Suit};
use poker_model::poker_deck::discard_cards;

#[test]
fn test_shuffle() {
    let mut shuffled_deck = Deck::new();
    shuffled_deck.shuffle();
    let mut unshuffled_deck = Deck::new();
    let s_cards = shuffled_deck.draw(52);
    let us_cards = unshuffled_deck.draw(52);
    assert_ne!(s_cards, us_cards);
}

#[test]
fn test_draw() {
    let mut deck = Deck::new();
    let mut hand = Vec::new();
    hand = deck.draw(2);
    assert_eq!(hand.len(), 2);
    assert_eq!(hand, 
               vec![Card{rank: Rank::A, suit: Suit::Spades},        // First two cards of unshuffled deck are A and King, both of Spades
                    Card{rank: Rank::King, suit: Suit::Spades}]);
}

#[test]
fn test_discard() {
    let mut spare_deck = Deck::new();
    let mut spare_cards = spare_deck.draw(1);
    spare_cards = spare_deck.draw(4);
    let mut deck = Deck::new();
    let mut hand = deck.draw(5);        // Unshuffled deck so hand looks like this: A K Q J 10
    discard_cards(&mut hand, "1");      // Should delete element at index 0
    assert_eq!(hand.len(), 4);
    assert_eq!(hand, spare_cards);
}

#[test]
fn test_discard_multiple() {
    let mut spare_deck = Deck::new();
    let _ = spare_deck.draw(1);         // Card that should be discarded
    let mut spare_cards = spare_deck.draw(1);
    _ = spare_deck.draw(1);             // Card that should be discarded
    spare_cards.extend(spare_deck.draw(2)); 
    
    let mut deck = Deck::new();
    let mut hand = deck.draw(5);        // Unshuffled deck so hand looks like this: A K Q J 10
    discard_cards(&mut hand, "1 3");      // Should delete element at index 0 and 2
    assert_eq!(hand.len(), 3);
    assert_eq!(hand, spare_cards);
}