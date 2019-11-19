use crate::game_state_parser;
use super::super::super::super::{Card, Move, Position, KING, QUEEN};
use super::super::super::super::Suit::{Club, Diamond, Heart, Spade};
use super::utils;



// cards can be moved to the freecells, but nothing else can be done
#[test]
fn test_easy() {
    let game_state = game_state_parser::parse_file("test-inputs/easy.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/ts-from-cascade0-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: 10 },
                from: Position::Cascade(0),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/6h-from-cascade1-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: 6 },
                from: Position::Cascade(1),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/kd-from-cascade2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/td-from-cascade3-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: 10 },
                from: Position::Cascade(3),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/qh-from-cascade4-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: QUEEN },
                from: Position::Cascade(4),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/4s-from-cascade5-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, rank: 4 },
                from: Position::Cascade(5),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/2c-from-cascade6-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: 2 },
                from: Position::Cascade(6),
                to: Position::Freecells,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy/6c-from-cascade7-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: 6 },
                from: Position::Cascade(7),
                to: Position::Freecells,
            }
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
