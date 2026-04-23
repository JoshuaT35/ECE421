/// Poker model of the deck

use rand::{
    seq::SliceRandom,
    thread_rng,
};
use std::fmt;
use serde::{Serialize, Deserialize};

/// A Card used in the deck
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Possible suites for a Card
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

/// Possible ranks for a Card
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    A,
}

/// Implement Display for Rank
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::A => "A",
        };
        write!(f, "{}", rank_str)
    }
}

/// Implement Display for Suit
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit_str = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit_str)
    }
}

/// Implement Display for Card (in a format suitable for TCP message)
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

/// Function to create a formatted message from a vector of cards
pub fn create_card_message(cards: &[Card]) -> String {
    cards.iter()
        .map(|card| format!("{}", card))  // Convert each card to a string
        .collect::<Vec<String>>()
        .join(", ")  // Join the cards with a comma separator
}

/// Function to discard cards in a given Vector of Cards by their index
pub fn discard_cards(player_hand: &mut Vec<Card>, input: &str) {
    // Parse the input string into a list of indices (1-based)
    let indices: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok()) // Parse numbers and filter out invalid ones
        .collect();

    // Sort indices in descending order to remove cards from the end (so earlier removals don't affect later ones)
    let mut sorted_indices = indices;
    sorted_indices.sort_by(|a, b| b.cmp(a));

    // Remove the cards from the hand
    for index in sorted_indices {
        if index >= 1 && index <= player_hand.len() {
            // Remove the card at the 0-based index (index - 1)
            player_hand.remove(index - 1);
        }
    }
}

/// The struct for a poker Deck
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Creates a new unshuffled deck of 52 cards.
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for &rank in &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::A,
            ] {
                cards.push(Card { rank, suit });
            }
        }
        let mut deck = Deck { cards };
        deck
    }

    /// Shuffles the deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draws `count` number of cards from the deck.
    pub fn draw(&mut self, count: usize) -> Vec<Card> {
        let mut drawn_cards = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.cards.pop() {
                drawn_cards.push(card);
            }
        }
        drawn_cards
    }
}
