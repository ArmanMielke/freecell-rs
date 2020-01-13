use freecell::Suit::{Club, Diamond, Heart};
use freecell::{Card, GameState, Move, Position, ACE};

use super::utils;

// only the cards in the freecells can be moved
#[test]
fn test_easy_full_freecells() {
    let game_state = GameState::from_file("test-inputs/easy-full-freecells.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            GameState::from_file("test-inputs/move-results/easy-full-freecells/6h-from-freecell3-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: 6 },
                from: Position::Freecell(3),
                to: Position::Cascade(1),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/easy-full-freecells/9d-from-freecell2-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: 9 },
                from: Position::Freecell(2),
                to: Position::Cascade(0),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/easy-full-freecells/9d-from-freecell2-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: 9 },
                from: Position::Freecell(2),
                to: Position::Cascade(3),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/easy-full-freecells/ac-from-freecell1-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: ACE },
                from: Position::Freecell(1),
                to: Position::Foundations,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/easy-full-freecells/ad-from-freecell0-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: ACE },
                from: Position::Freecell(0),
                to: Position::Foundations,
            },
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
