use std::net::TcpStream;
use std::thread;

use poker_model::game_types::dealer_game::DealerGame;
use poker_model::game_types::five_card_draw::FiveCardDrawDealer;
use poker_model::poker_players::poker_player_base::{PlayerStatus, PokerPlayer};
use poker_model::poker_players::poker_player_five_card_draw::PokerPlayerFiveCardDraw;
use poker_server::game_logic::game_utils::player_pays_dealer;

#[test]
// Tests that betting rounds increment pot size correctly
fn test_betting_adds_to_pot() {
    let mut dealer = FiveCardDrawDealer::new();
    let mut player = PokerPlayerFiveCardDraw::new("TestPlayer");
    
    // Initial pot and player chips
    assert_eq!(dealer.get_pot(), 0);
    assert_eq!(player.get_num_chips(), 10);
    
    // Player pays ante
    player.pay(1);
    dealer.add_to_pot(1);
    
    // Check pot and player chips after ante
    assert_eq!(dealer.get_pot(), 1);
    assert_eq!(player.get_num_chips(), 9);
}

#[test]
// Tests player status transitions
fn test_player_status_transitions() {
    let mut player = PokerPlayerFiveCardDraw::new("TestPlayer");
    
    // Player starts Active
    assert_eq!(player.get_status(), PlayerStatus::Active);
    
    // Set status to various states
    player.set_status(PlayerStatus::Checked);
    assert_eq!(player.get_status(), PlayerStatus::Checked);
    
    player.set_status(PlayerStatus::Called);
    assert_eq!(player.get_status(), PlayerStatus::Called);
    
    player.set_status(PlayerStatus::Raised);
    assert_eq!(player.get_status(), PlayerStatus::Raised);
    
    player.set_status(PlayerStatus::Folded);
    assert_eq!(player.get_status(), PlayerStatus::Folded);
}

#[test]
// Tests proper card discard and replacement functionality
fn test_discard_and_replace() {
    let mut dealer = FiveCardDrawDealer::new();
    let mut player = PokerPlayerFiveCardDraw::new("TestPlayer");
    
    // Deal initial hand
    dealer.shuffle_deck();
    player.receive_cards(dealer.deal_cards(5));
    assert_eq!(player.get_num_cards(), 5);
    
    // Get original hand for comparison
    let original_hand = player.get_hand();
    
    // Discard first and third cards (0-indexed)
    poker_model::poker_deck::discard_cards(&mut player.hand, "1 3");
    
    // Verify discarded
    assert_eq!(player.get_num_cards(), 3);
    
    // Replace discarded cards
    player.receive_cards(dealer.deal_cards(2));
    
    // Verify hand size and difference from original
    assert_eq!(player.get_num_cards(), 5);
    assert_ne!(player.get_hand(), original_hand);
}