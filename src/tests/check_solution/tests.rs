use freecell::{Card, Move, Position};
use freecell::card::KING;
use freecell::card::Suit::*;
use game_state_parser::parse_file;
use super::check_solution;



#[test]
fn test_correct_solution_a() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();

    let solution = vec![
        Move{
            card: Card { suit: Spade, value: KING },
            from: Position::Cascade(1),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Club, value: KING },
            from: Position::Cascade(2),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Diamond, value: KING },
            from: Position::Cascade(5),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Cascade(7),
            to: Position::Foundations,
        },
    ];

    check_solution(solution, initial_state, 4)
}

#[test]
fn test_correct_solution_b() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();

    let solution = vec![
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Cascade(7),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Diamond, value: KING },
            from: Position::Cascade(5),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Club, value: KING },
            from: Position::Cascade(2),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Spade, value: KING },
            from: Position::Cascade(1),
            to: Position::Foundations,
        },
    ];

    check_solution(solution, initial_state, 4)
}

#[test]
#[should_panic(expected = "Solution has length 5, should have length 4")]
fn test_wrong_length() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();

    let solution = vec![
        Move{
            card: Card { suit: Spade, value: KING },
            from: Position::Cascade(1),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Club, value: KING },
            from: Position::Cascade(2),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Diamond, value: KING },
            from: Position::Cascade(5),
            to: Position::Foundations,
        },
        // suboptimal move
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Cascade(7),
            to: Position::Freecells,
        },
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Freecells,
            to: Position::Foundations,
        },
    ];

    check_solution(solution, initial_state, 4)
}

#[test]
#[should_panic(expected = "is not legal at this point in the game.")]
fn test_illegal_move() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();
    let solution = vec![
        Move{
            card: Card { suit: Spade, value: KING },
            from: Position::Cascade(1),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Club, value: KING },
            from: Position::Cascade(2),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Diamond, value: KING },
            from: Position::Cascade(5),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Cascade(1), // the card is actually in cascade 7
            to: Position::Foundations,
        },
    ];

    check_solution(solution, initial_state, 4)
}

#[test]
#[should_panic(expected = "Following the solution should lead to the solved state, but instead leads to")]
fn test_does_not_lead_to_solved_state() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();

    let solution = vec![
        Move{
            card: Card { suit: Spade, value: KING },
            from: Position::Cascade(1),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Club, value: KING },
            from: Position::Cascade(2),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Diamond, value: KING },
            from: Position::Cascade(5),
            to: Position::Foundations,
        },
        Move{
            card: Card { suit: Heart, value: KING },
            from: Position::Cascade(7),
            to: Position::Freecells, // the game is not solved yet
        },
    ];

    check_solution(solution, initial_state, 4)
}
