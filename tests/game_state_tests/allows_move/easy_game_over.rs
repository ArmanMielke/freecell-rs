use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, GameState, Move, POSITIONS};

#[test]
fn test_easy_game_over() {
    let game_state = GameState::from_file("test-inputs/easy-game-over.txt").unwrap();

    // for every single card, ...
    for &suit in &[Club, Diamond, Heart, Spade] {
        for rank in 1..=13 {
            let card = Card { suit, rank };

            // ... every single source position
            for &from in &POSITIONS {
                // ... and every single target position
                for &to in &POSITIONS {
                    // .. moving that card from the source position to the target position is illegal
                    assert!(!game_state.allows_move(Move { card, from, to }))
                }
            }
        }
    }
}
