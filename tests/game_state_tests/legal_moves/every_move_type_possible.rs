use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, GameState, Move, Position, JACK, KING, QUEEN};

use super::utils;

#[test]
fn test_every_move_type_possible() {
    // all move types are possible in this game state:
    // - cascade -> cascade
    // - cascade -> foundations
    // - cascade -> freecells
    // - freecells -> cascade
    // - freecells -> foundations

    let game_state = GameState::from_file("test-inputs/every-move-type-possible.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jc-from-freecells-to-cascade-6.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: JACK },
                from: Position::Freecells,
                to: Position::Cascade(6),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jc-from-freecells-to-cascade-7.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: JACK },
                from: Position::Freecells,
                to: Position::Cascade(7),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jc-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: JACK },
                from: Position::Freecells,
                to: Position::Foundations,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jd-from-cascade-0-to-cascade-4.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: JACK },
                from: Position::Cascade(0),
                to: Position::Cascade(4),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jd-from-cascade-0-to-cascade-5.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: JACK },
                from: Position::Cascade(0),
                to: Position::Cascade(5),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jd-from-cascade-0-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: JACK },
                from: Position::Cascade(0),
                to: Position::Foundations,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/jd-from-cascade-0-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: JACK },
                from: Position::Cascade(0),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/kd-from-cascade-3-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: KING },
                from: Position::Cascade(3),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/kh-from-cascade-2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/ks-from-cascade-1-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: KING },
                from: Position::Cascade(1),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qc-from-cascade-4-to-cascade-2.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: QUEEN },
                from: Position::Cascade(4),
                to: Position::Cascade(2),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qc-from-cascade-4-to-cascade-3.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: QUEEN },
                from: Position::Cascade(4),
                to: Position::Cascade(3),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qc-from-cascade-4-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: QUEEN },
                from: Position::Cascade(4),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qd-from-cascade-7-to-cascade-1.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: QUEEN },
                from: Position::Cascade(7),
                to: Position::Cascade(1),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qd-from-cascade-7-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: QUEEN },
                from: Position::Cascade(7),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qh-from-cascade-6-to-cascade-1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: QUEEN },
                from: Position::Cascade(6),
                to: Position::Cascade(1),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qh-from-cascade-6-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: QUEEN },
                from: Position::Cascade(6),
                to: Position::Freecells,
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qs-from-cascade-5-to-cascade-2.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: QUEEN },
                from: Position::Cascade(5),
                to: Position::Cascade(2),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qs-from-cascade-5-to-cascade-3.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: QUEEN },
                from: Position::Cascade(5),
                to: Position::Cascade(3),
            },
        ),
        (
            GameState::from_file("test-inputs/move-results/every-move-type-possible/qs-from-cascade-5-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: QUEEN },
                from: Position::Cascade(5),
                to: Position::Freecells,
            },
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
