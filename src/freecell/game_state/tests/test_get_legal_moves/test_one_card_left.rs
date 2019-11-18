use crate::game_state_parser;
use super::super::super::super::{Card, Move, Position, KING};
use super::super::super::super::Suit::Club;
use super::utils;



#[test]
fn test_one_card_left() {
    let game_state = game_state_parser::parse_file("test-inputs/one-card-left.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(0),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(1),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(3),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade4.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(4),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(5),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade6.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(6),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade7.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(7),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Foundations,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            }
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
