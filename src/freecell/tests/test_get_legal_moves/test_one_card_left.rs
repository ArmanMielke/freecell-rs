use game_state_parser::parse_file;
use super::utils::assert_eq_ignore_order;
use super::super::super::Move;
use super::super::super::card::*;
use super::super::super::card::Suit::*;
use super::super::super::position::Position;



#[test]
fn test_one_card_left() {
    let game_state = parse_file("test-inputs/one-card-left.txt").unwrap();
    let actual = game_state.get_legal_moves();
    let expected = vec![
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(0),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(1),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade3.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(3),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade4.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(4),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(5),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade6.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(6),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-cascade7.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Cascade(7),
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-foundations.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Foundations,
            }
        ),
        (
            parse_file("test-inputs/move-results/one-card-left/kc-from-cascade2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            }
        ),
    ];

    assert_eq_ignore_order(actual, expected);
}
