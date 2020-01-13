use freecell::Suit::Club;
use freecell::{Card, GameState, Move, Position, KING};

use super::utils;

#[test]
fn test_one_card_left() {
    let game_state = GameState::from_file("test-inputs/one-card-left.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(0),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(1),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(3),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade4.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(4),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(5),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade6.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(6),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade7.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(7),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Foundations,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell0.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(0),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell1.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(1),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell2.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(2),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecell3.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecell(3),
            },
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
