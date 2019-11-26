use crate::game_state_parser;
use super::super::super::super::{Card, Move, Position};
use super::super::super::super::Suit::{Club, Diamond, Heart, Spade};

const POSITIONS: [Position; 10] = [
    Position::Cascade(0),
    Position::Cascade(1),
    Position::Cascade(2),
    Position::Cascade(3),
    Position::Cascade(4),
    Position::Cascade(5),
    Position::Cascade(6),
    Position::Cascade(7),
    Position::Foundations,
    Position::Freecells,
];

#[test]
fn test_easy_game_over() {
    let game_state = game_state_parser::parse_file("test-inputs/easy-game-over.txt").unwrap();

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
