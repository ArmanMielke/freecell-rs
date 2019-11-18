use crate::game_state_parser;
use super::super::super::card::{Card, ACE};
use super::super::super::card::Suit::{Club, Diamond, Heart};
use super::super::super::Move;
use super::super::super::position::Position;
use super::utils;



// only the cards in the freecells can be moved
#[test]
fn test_easy_full_freecells() {
    let game_state = game_state_parser::parse_file("test-inputs/easy-full-freecells.txt").unwrap();
    let actual = game_state.get_legal_moves();
    let expected = vec![
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/6h-from-freecells-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, value: 6 },
                from: Position::Freecells,
                to: Position::Cascade(1),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: 9 },
                from: Position::Freecells,
                to: Position::Cascade(0),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: 9 },
                from: Position::Freecells,
                to: Position::Cascade(3),
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/ac-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            }
        ),
        (
            game_state_parser::parse_file("test-inputs/move-results/easy-full-freecells/ad-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            }
        ),
    ];

    utils::assert_eq_ignore_order(actual, expected);
}
