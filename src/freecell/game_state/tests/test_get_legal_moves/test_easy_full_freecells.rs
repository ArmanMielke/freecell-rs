use super::super::super::super::Suit::{Club, Diamond, Heart};
use super::super::super::super::{Card, Move, Position, ACE};
use super::utils;
use crate::game_state_parser;

// only the cards in the freecells can be moved
#[test]
fn test_easy_full_freecells() {
    let game_state = game_state_parser::parse_file("test-inputs/easy-full-freecells.txt").unwrap();
    let actual = game_state.legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/6h-from-freecells-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, rank: 6 },
                from: Position::Freecells,
                to: Position::Cascade(1),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: 9 },
                from: Position::Freecells,
                to: Position::Cascade(0),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: 9 },
                from: Position::Freecells,
                to: Position::Cascade(3),
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/ac-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, rank: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            },
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/ad-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, rank: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            },
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
