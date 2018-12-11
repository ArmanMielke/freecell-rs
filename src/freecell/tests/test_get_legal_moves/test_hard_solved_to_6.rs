use game_state_parser::parse_file;
use super::utils::assert_eq_ignore_order;
use super::super::super::Move;
use super::super::super::card::*;
use super::super::super::card::Suit::*;
use super::super::super::position::Position;



#[test]
fn test_hard_solved_to_6() {
    let game_state = parse_file("test-inputs/hard-solved-to-6.txt").unwrap();
    let actual = game_state.get_legal_moves();
    let expected = vec![
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/8s-from-cascade6-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, value: 8 },
                from: Position::Cascade(6),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/9s-from-cascade3-to-cascade0.txt").unwrap(),
            Move {
                card: Card { suit: Spade, value: 9 },
                from: Position::Cascade(3),
                to: Position::Cascade(0),
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/9s-from-cascade3-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Spade, value: 9 },
                from: Position::Cascade(3),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/jc-from-cascade1-to-cascade5.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: JACK },
                from: Position::Cascade(1),
                to: Position::Cascade(5),
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/jc-from-cascade1-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: JACK },
                from: Position::Cascade(1),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/jd-from-cascade4-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: JACK },
                from: Position::Cascade(4),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/kc-from-cascade2-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Club, value: KING },
                from: Position::Cascade(2),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/kh-from-cascade7-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, value: KING },
                from: Position::Cascade(7),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/qd-from-cascade5-to-cascade2.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: QUEEN },
                from: Position::Cascade(5),
                to: Position::Cascade(2),
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/qd-from-cascade5-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Diamond, value: QUEEN },
                from: Position::Cascade(5),
                to: Position::Freecells,
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/th-from-cascade0-to-cascade1.txt").unwrap(),
            Move {
                card: Card { suit: Heart, value: 10 },
                from: Position::Cascade(0),
                to: Position::Cascade(1),
            }
        ),
        (
            parse_file("test-inputs/move-results/hard-solved-to-6/th-from-cascade0-to-freecells.txt").unwrap(),
            Move {
                card: Card { suit: Heart, value: 10 },
                from: Position::Cascade(0),
                to: Position::Freecells,
            }
        ),
    ];

    assert_eq_ignore_order(actual, expected);
}
