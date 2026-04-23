use poker_model::{
    game_types::texas_holdem::TexasHoldemDealer,
    game_types::dealer_game::DealerGame,
    poker_players::poker_player_texas_holdem::PokerPlayerTexasHoldem,
    poker_players::poker_player_base::{PokerPlayer, PlayerStatus},
    poker_deck::{Card, Rank, Suit},
};
use poker_server::game_logic::game_utils;


fn make_test_player(name: &str) -> PokerPlayerTexasHoldem {
    let mut player = PokerPlayerTexasHoldem::new(name);
    player
}


fn make_dummy_stream() -> std::net::TcpStream {
    use std::net::{TcpListener, TcpStream};
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = std::thread::spawn(move || TcpStream::connect(addr).unwrap());
    listener.accept().unwrap().0
}


#[test]
fn test_num_active_players() {
    let mut players = vec![
        (0, make_test_player("A"), make_dummy_stream()),
        (1, make_test_player("B"), make_dummy_stream()),
    ];
    players[1].1.set_status(PlayerStatus::Folded);

    let count = game_utils::num_active_players(&players);
    assert_eq!(count, 1);
}

#[test]
fn test_player_pays_dealer_reduces_chips_and_increases_pot() {
    let mut dealer = TexasHoldemDealer::new();
    let mut player = make_test_player("Player");

    game_utils::player_pays_dealer(&mut dealer, &mut player, 2);

    assert_eq!(player.get_num_chips(), 8);
    assert_eq!(dealer.get_pot(), 2);
}

#[test]
fn test_evaluate_winner_selects_best_hand() {
    let mut players = vec![
        (0, make_test_player("A"), make_dummy_stream()),
        (1, make_test_player("B"), make_dummy_stream()),
    ];

    // Give player A a high card hand
    players[0].1.receive_cards(vec![
        Card { rank: Rank::A, suit: Suit::Spades },
        Card { rank: Rank::King, suit: Suit::Spades },
        Card { rank: Rank::Queen, suit: Suit::Spades },
        Card { rank: Rank::Jack, suit: Suit::Hearts },
        Card { rank: Rank::Nine, suit: Suit::Clubs },
    ]);

    // Give player B a lower hand
    players[1].1.receive_cards(vec![
        Card { rank: Rank::Three, suit: Suit::Diamonds },
        Card { rank: Rank::Four, suit: Suit::Hearts },
        Card { rank: Rank::Six, suit: Suit::Clubs },
        Card { rank: Rank::Seven, suit: Suit::Clubs },
        Card { rank: Rank::Eight, suit: Suit::Hearts },
    ]);

    let (winners, _) = game_utils::evaluate_winner(&mut players);
    assert_eq!(winners, vec!["A"]);
}


#[test]
fn test_all_fold_returns_last_non_folded() {
    let mut players = vec![
        (0, make_test_player("A"), make_dummy_stream()),
        (1, make_test_player("B"), make_dummy_stream()),
    ];
    players[0].1.set_status(PlayerStatus::Folded);
    players[1].1.set_status(PlayerStatus::Active);

    let (winners, _) = game_utils::all_fold(&mut players);
    assert_eq!(winners, vec!["B"]);
}

#[test]
fn test_distribute_pot_splits_evenly() {
    let mut dealer = TexasHoldemDealer::new();
    dealer.add_to_pot(10);
    assert_eq!(dealer.get_pot(), 10);

    // test players initialized with 10 chips based on current implementation
    let mut p1 = make_test_player("P1");
    let mut p2 = make_test_player("P2");

    let mut players = vec![
        (0, p1, make_dummy_stream()),
        (1, p2, make_dummy_stream()),
    ];

    let winner_names = vec!["P1".to_string(), "P2".to_string()];
    game_utils::distribute_pot(&mut dealer, &mut players, &winner_names);

    assert_eq!(players[0].1.get_num_chips(), 15);
    assert_eq!(players[1].1.get_num_chips(), 15);
    // pot distributed so should be 0
    assert_eq!(dealer.get_pot(), 0);
}

#[test]
fn test_go_to_next_round_resets_players_and_dealer() {
    let mut dealer = TexasHoldemDealer::new();

    let mut players = vec![
        (0, make_test_player("P1"), make_dummy_stream()),
        (1, make_test_player("P2"), make_dummy_stream()),
    ];
    // pays 5 chips to dealer
    game_utils::player_pays_dealer(&mut dealer, &mut players[0].1, 5);
    assert_eq!(dealer.get_pot(), 5);
    let keep_going = game_utils::go_to_next_round_and_set_environment(&mut dealer, &mut players);
    assert_eq!(dealer.get_pot(), 0);
    assert!(keep_going);
    
    // pays remaining 5 chips to dealer
    game_utils::player_pays_dealer(&mut dealer, &mut players[0].1, 5);
    let keep_going = game_utils::go_to_next_round_and_set_environment(&mut dealer, &mut players);
    assert!(!keep_going);

    assert_eq!(players[0].1.get_status(), PlayerStatus::Eliminated);
    assert_eq!(players[1].1.get_status(), PlayerStatus::Active);
}
