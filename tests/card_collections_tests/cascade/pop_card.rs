use freecell::Suit::{Diamond, Heart, Spade};
use freecell::{Card, CardCollection, Cascade, GameState};

#[test]
fn test_empty() {
    let cascade: Cascade = Vec::new();
    assert_eq!(cascade.pop_card(), Vec::new());
}

#[test]
fn test_not_empty() {
    let game_state = GameState::from_file("test-inputs/hard-solved-to-5.txt").unwrap();
    let expected = vec![
        (
            vec![
                Card { suit: Diamond, rank: 10 },
                Card { suit: Heart, rank: 9 },
                Card { suit: Spade, rank: 9 },
                Card { suit: Diamond, rank: 6 },
            ],
            Card { suit: Spade, rank: 6 }
        )
    ];
    assert_eq!(game_state.cascades[3].pop_card(), expected);
}
