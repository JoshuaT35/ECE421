/// contains the possible Poker hands, and ways to evaluate them

use std::cmp::Ordering;
use std::collections::HashMap;
use crate::poker_deck::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum HandRank {
    RoyalFlush, // No tiebreaker needed, always the best
    StraightFlush(Rank), // Highest card in the straight flush
    FourOfAKind(Rank, Rank), // (Quad value, Kicker)
    FullHouse(Rank, Rank), // (Three of a kind value, Pair value)
    Flush(Vec<Rank>, Suit), // Sorted list of flush cards
    Straight(Rank), // Highest card in the straight
    ThreeOfAKind(Rank, Vec<Rank>), // (Triplet value, Kickers)
    TwoPair(Rank, Rank, Rank), // (High pair, Low pair, Kicker)
    OnePair(Rank, Vec<Rank>), // (Pair value, Kickers)
    HighCard(Vec<Rank>), // Sorted kickers
}
impl HandRank {
    fn get_rank_value(&self) -> u8 {
        match self {
            HandRank::RoyalFlush => 10,
            HandRank::StraightFlush(_) => 9,
            HandRank::FourOfAKind(_, _) => 8,
            HandRank::FullHouse(_, _) => 7,
            HandRank::Flush(_, _) => 6,
            HandRank::Straight(_) => 5,
            HandRank::ThreeOfAKind(_, _) => 4,
            HandRank::TwoPair(_, _, _) => 3,
            HandRank::OnePair(_, _) => 2,
            HandRank::HighCard(_) => 1,
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        // First, compare the rank of the hand itself (Flush should always beat Straight)
        let self_value = self.get_rank_value();
        let other_value = other.get_rank_value();

        if self_value != other_value {
            return self_value.cmp(&other_value); // Directly return the ordering if ranks differ
        }
    
        match self_value.cmp(&other_value) {
            Ordering::Equal => {
                // If hand types are the same, apply tie-breaking rules
                match (self, other) {
                    (HandRank::StraightFlush(a), HandRank::StraightFlush(b)) => a.cmp(b),
                    (HandRank::FourOfAKind(a, k1), HandRank::FourOfAKind(b, k2)) => a.cmp(b).then(k1.cmp(k2)),
                    (HandRank::FullHouse(a1, b1), HandRank::FullHouse(a2, b2)) => a1.cmp(a2).then(b1.cmp(b2)),
                    (HandRank::Flush(c1, _), HandRank::Flush(c2, _)) => c1.cmp(c2),
                    (HandRank::Straight(a), HandRank::Straight(b)) => a.cmp(b),
                    (HandRank::ThreeOfAKind(a, k1), HandRank::ThreeOfAKind(b, k2)) => a.cmp(b).then(k1.cmp(k2)),
                    (HandRank::TwoPair(hp1, lp1, k1), HandRank::TwoPair(hp2, lp2, k2)) => {
                        hp1.cmp(hp2).then(lp1.cmp(lp2)).then(k1.cmp(k2))
                    }
                    (HandRank::OnePair(p1, k1), HandRank::OnePair(p2, k2)) => p1.cmp(p2).then(k1.cmp(k2)),
                    (HandRank::HighCard(k1), HandRank::HighCard(k2)) => k1.cmp(k2),
                    _ => Ordering::Equal,
                }
            }
            other => other, // If different rank types, return the comparison immediately
        }
    }
}

pub struct HandEvaluator;

impl HandEvaluator {
    pub fn evaluate_hand(cards: &Vec<Card>) -> HandRank {
        if let Some(rank) = Self::is_royal_flush(cards) { return rank; }
        if let Some(rank) = Self::is_straight_flush(cards) { return rank; }
        if let Some(rank) = Self::is_four_of_a_kind(cards) { return rank; }
        if let Some(rank) = Self::is_full_house(cards) { return rank; }
        if let Some(rank) = Self::is_flush(cards) { return rank; }
        if let Some(rank) = Self::is_straight(cards) { return rank; }
        if let Some(rank) = Self::is_three_of_a_kind(cards) { return rank; }
        if let Some(rank) = Self::is_two_pair(cards) { return rank; }
        if let Some(rank) = Self::is_one_pair(cards) { return rank; }

        let mut kickers: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
        kickers.sort_by(|a, b| b.cmp(a));
        HandRank::HighCard(kickers)
    }

    pub fn rank_hands(player_cards: Vec<(String, Vec<Card>)>) -> (Vec<String>, Vec<Card>) {
        let mut best_players = vec![];
        let mut best_rank: Option<HandRank> = None;
        let mut best_hand: Option<Vec<Card>> = None;
    
        for (name, hand) in player_cards {
            let rank = HandEvaluator::evaluate_hand(&hand);
    
            match &best_rank {
                None => {
                    best_rank = Some(rank.clone());
                    best_hand = Some(hand.clone());
                    best_players.push(name);
                }
                Some(r) => match rank.compare(r) {
                    Ordering::Greater => {
                        best_rank = Some(rank.clone());
                        best_hand = Some(hand.clone());
                        best_players = vec![name];
                    }
                    Ordering::Equal => {
                        best_players.push(name);
                    }
                    Ordering::Less => {}
                },
            }
        }
    
        (best_players, best_hand.unwrap())
    }
    
    pub fn worst_hand(player_cards: Vec<(String, Vec<Card>)>) -> (Vec<String>, Vec<Card>) {
        let mut worst_players = vec![];
        let mut worst_rank: Option<HandRank> = None;
        let mut worst_hand: Option<Vec<Card>> = None;
    
        for (name, hand) in player_cards {
            let rank = HandEvaluator::evaluate_hand(&hand);
    
            match &worst_rank {
                None => {
                    worst_rank = Some(rank.clone());
                    worst_hand = Some(hand.clone());
                    worst_players.push(name);
                }
                Some(r) => match rank.compare(r) {
                    Ordering::Less => {
                        worst_rank = Some(rank.clone());
                        worst_hand = Some(hand.clone());
                        worst_players = vec![name];
                    }
                    Ordering::Equal => {
                        worst_players.push(name);
                    }
                    Ordering::Greater => {}
                },
            }
        }
    
        (worst_players, worst_hand.unwrap())
    }

    // given a vector of 5 cards or more, return the best 5-hand in the vector
    pub fn best_five_hand_from_larger(larger_hand: Vec<Card>) -> Vec<Card> {
        let mut best_rank: Option<HandRank> = None;
        let mut best_hand = vec![];
    
        for combo in larger_hand.iter().copied().combinations(5) {
            let rank = HandEvaluator::evaluate_hand(&combo);
            if best_rank.is_none() || rank.compare(best_rank.as_ref().unwrap()) == Ordering::Greater {
                best_rank = Some(rank);
                best_hand = combo;
            }
        }

        best_hand
    }

    fn is_royal_flush(cards: &Vec<Card>) -> Option<HandRank> {
        // checks to make sure it is first a straight flush 
        // doesn't need an explicit check for cards.len() == 5 as
        // this is checked in is_straight_flush()
        if let Some(HandRank::StraightFlush(highest)) = Self::is_straight_flush(cards) {
            let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
            ranks.sort(); // Sort the ranks
            // checks for specifically 10 J Q K A straight
            let high_straight = vec![Rank::A, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
            if high_straight.iter().all(|rank| ranks.contains(rank)) {
                return Some(HandRank::RoyalFlush);
            }
        }
        None
    }
    
    fn is_straight_flush(cards: &Vec<Card>) -> Option<HandRank> {
        if cards.len() == 5 {
            // Group by suit
            let mut suits: HashMap<Suit, Vec<Card>> = HashMap::new();
            for &card in cards {
                suits.entry(card.suit).or_default().push(card);
            }
        
            for suited_cards in suits.values() {
                let mut ranks: Vec<Rank> = suited_cards.iter().map(|c| c.rank).collect();
                ranks.sort();
                ranks.dedup();
    
                // Check for standard straight
                for window in ranks.windows(5) {
                    if window[0] as u8 + 1 == window[1] as u8 &&
                        window[1] as u8 + 1 == window[2] as u8 &&
                        window[2] as u8 + 1 == window[3] as u8 &&
                        window[3] as u8 + 1 == window[4] as u8 {
                        return Some(HandRank::StraightFlush(*window.last().unwrap()));
                    }
                }
    
                // Check for Ace-low straight
                let low_straight = vec![Rank::A, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
                if low_straight.iter().all(|r| ranks.contains(r)) {
                    return Some(HandRank::StraightFlush(Rank::Five));
                }
            }
        }
    
        None
    }
    

    fn is_flush(cards: &Vec<Card>) -> Option<HandRank> {
        if cards.len() == 5 {
            // Group cards by suit
            let mut suits: HashMap<Suit, Vec<Rank>> = HashMap::new();
        
            for card in cards {
                suits.entry(card.suit).or_default().push(card.rank);
            }
        
            // Look for any suit with 5 or more cards
            for (suit, mut ranks) in suits {
                if ranks.len() >= 5 {
                    ranks.sort_by(|a, b| b.cmp(a));
                    return Some(HandRank::Flush(ranks[..5].to_vec(), suit));
                }
            }
        }

        None
    }
    

    fn is_four_of_a_kind(cards: &Vec<Card>) -> Option<HandRank> {
        let mut rank_counts = HashMap::new();
        for card in cards { *rank_counts.entry(card.rank).or_insert(0) += 1; }
        let mut quad = None;
        let mut kicker = None;
        for (&rank, &count) in &rank_counts {
            if count == 4 { quad = Some(rank); }
            else { kicker = Some(rank); }
        }
        if let Some(quad) = quad {
            return Some(HandRank::FourOfAKind(quad, kicker.unwrap_or(Rank::Two)));
        }
        None
    }

    fn is_full_house(cards: &Vec<Card>) -> Option<HandRank> {
        if cards.len() == 5 {
            let mut rank_counts = HashMap::new();
            for card in cards {
                *rank_counts.entry(card.rank).or_insert(0) += 1;
            }
    
            let mut three = None;
            let mut two = None;
    
            for (&rank, &count) in &rank_counts {
                if count == 3 {
                    three = Some(rank);
                } else if count == 2 {
                    two = Some(rank);
                }
            }
    
            if let (Some(three), Some(two)) = (three, two) {
                return Some(HandRank::FullHouse(three, two));
            }
        }

        None
    }

    fn is_straight(cards: &Vec<Card>) -> Option<HandRank> {
        if cards.len() == 5{
            let mut ranks: Vec<Rank> = cards.iter().map(|card| card.rank).collect();
            ranks.sort(); // Sort the ranks
            
            // Check if the ranks form a sequence
            let is_straight = ranks.windows(2).all(|w| w[0] as u8 + 1 == w[1] as u8);
            
            if is_straight {
                return Some(HandRank::Straight(*ranks.last().unwrap())); 
            }
        
            // Check for Ace-low straight (A, 2, 3, 4, 5)
            let low_straight = vec![Rank::A, Rank::Two, Rank::Three, Rank::Four, Rank::Five];
            if low_straight.iter().all(|rank| ranks.contains(rank)) {
                return Some(HandRank::Straight(Rank::Five));
            }
        }        
        None
    }

    fn is_three_of_a_kind(cards: &Vec<Card>) -> Option<HandRank> {

        if cards.len() >= 3 {
            let mut rank_counts = HashMap::new();
            for card in cards {
                *rank_counts.entry(card.rank).or_insert(0) += 1;
            }
        
            let mut three = None;
            let mut kickers = Vec::new();
        
            for (&rank, &count) in &rank_counts {
                if count == 3 {
                    three = Some(rank);
                } else {
                    kickers.push(rank);
                }
            }
        
            if let Some(three_rank) = three {
                kickers.sort_by(|a, b| b.cmp(a));
                return Some(HandRank::ThreeOfAKind(three_rank, kickers));
            }
        }
    
        None
    }
    
    fn is_two_pair(cards: &Vec<Card>) -> Option<HandRank> {
        let mut rank_counts = HashMap::new();
        for card in cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }
    
        let mut pairs = Vec::new();
        let mut kicker = None;
    
        for (&rank, &count) in &rank_counts {
            if count == 2 {
                pairs.push(rank);
            } else if kicker.is_none() {
                kicker = Some(rank);
            }
        }
    
        if pairs.len() == 2 {
            pairs.sort_by(|a, b| b.cmp(a)); // Sort highest pair first
            return Some(HandRank::TwoPair(pairs[0], pairs[1], kicker.unwrap_or(Rank::Two)));
        }
    
        None
    }

    pub fn is_one_pair(cards: &Vec<Card>) -> Option<HandRank> {
        let mut value_counts = std::collections::HashMap::new();
        
        for card in cards {
            *value_counts.entry(card.rank).or_insert(0) += 1;
        }
    
        let mut pair_value = None;
        let mut kickers = Vec::new();
    
        for (&value, &count) in &value_counts {
            if count == 2 {
                pair_value = Some(value);
            } else {
                kickers.push(value);
            }
        }
    
        if let Some(pair) = pair_value {
            kickers.sort_by(|a, b| b.cmp(a)); // Sort kickers descending
            return Some(HandRank::OnePair(pair, kickers));
        }
    
        None // No pair found
    }
}
