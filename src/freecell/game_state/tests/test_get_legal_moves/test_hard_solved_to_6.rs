use crate::game_state_parser;
use super::super::super::super::{Card, Move, Position, JACK, KING, QUEEN};
use super::super::super::super::Suit::{Club, Diamond, Heart, Spade};
use super::utils;



#[test]
fn test_hard_solved_to_6() {
    let game_state = game_state_parser::parse_file("test-inputs/hard-solved-to-6.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/8s-from-cascade6-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: 8 },
                from: Position::Cascade(6),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/9s-from-cascade3-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: 9 },
                from: Position::Cascade(3),
                to: Position::Cascade(0),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/9s-from-cascade3-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: 9 },
                from: Position::Cascade(3),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/jc-from-cascade1-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: JACK },
                from: Position::Cascade(1),
                to: Position::Cascade(5),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/jc-from-cascade1-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: JACK },
                from: Position::Cascade(1),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/jd-from-cascade4-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: JACK },
                from: Position::Cascade(4),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/kc-from-cascade2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/kh-from-cascade7-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: KING },
                from: Position::Cascade(7),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/qd-from-cascade5-to-cascade2.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: QUEEN },
                from: Position::Cascade(5),
                to: Position::Cascade(2),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/qd-from-cascade5-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: QUEEN },
                from: Position::Cascade(5),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/th-from-cascade0-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: 10 },
                from: Position::Cascade(0),
                to: Position::Cascade(1),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/hard-solved-to-6/th-from-cascade0-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: 10 },
                from: Position::Cascade(0),
                to: Position::Freecells,
            }
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
