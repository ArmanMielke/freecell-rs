use game_state_parser::parse_file;
use super::utils::assert_eq_ignore_order;
use super::super::super::Move;
use super::super::super::card::*;
use super::super::super::card::Suit::*;
use super::super::super::position::Position;



// only the cards in the freecells can be moved
#[test]
fn test_easy_full_freecells() {
    let game_state = parse_file("test-inputs/easy-full-freecells.txt").unwrap();
    let actual = game_state.get_legal_moves();
    let expected = vec![
        (
            parse_file("test-inputs/move-results/easy-full-freecells/6h-from-freecells-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, value: 6 },
                from: Position::Freecells,
                to: Position::Cascade(1),
            }
        ),
        (
            parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: 9 },
                from: Position::Freecells,
                to: Position::Cascade(0),
            }
        ),
        (
            parse_file("test-inputs/move-results/easy-full-freecells/9d-from-freecells-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: 9 },
                from: Position::Freecells,
                to: Position::Cascade(3),
            }
        ),
        (
            parse_file("test-inputs/move-results/easy-full-freecells/ac-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            }
        ),
        (
            parse_file("test-inputs/move-results/easy-full-freecells/ad-from-freecells-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: ACE },
                from: Position::Freecells,
                to: Position::Foundations,
            }
        ),
    ];

    assert_eq_ignore_order(actual, expected);
}
