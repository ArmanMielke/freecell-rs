use freecell::{GameState, Move};



/// Checks whether the solution has the correct length and whether it's a valid sequence of moves
/// that leads from the initial state to the solved state.
pub fn check_solution(solution: Vec<Move>, initial_state: GameState, correct_length: usize) {
    check_solution_length(&solution, correct_length);
    check_solution_correctness(solution, initial_state)
}

fn check_solution_length(solution: &Vec<Move>, correct_length: usize) {
    if solution.len() != correct_length {
        panic!("Solution has length {}, should have length {}", solution.len(), correct_length);
    }
}

/// Checks whether the solution is a valid sequence of moves that leads to the solved state.
fn check_solution_correctness(solution: Vec<Move>, initial_state: GameState) {
    let mut current_state = initial_state;

    // check whether each of the moves is legal at that point in the game
    for (move_number, solution_move) in solution.iter().enumerate() {
        let mut next_state = None;

        for (resulting_state, legal_move) in current_state.get_legal_moves() {
            if solution_move.eq(&legal_move) {
                next_state = Some(resulting_state);
                break;
            }
        }

        // if next_state is None that means the solution's move did not match any of the legal moves
        current_state = match next_state {
            Some(game_state) => game_state,
            None => panic!(
                "Move {} is not legal at this point in the game.\nMove: {}\nGameState: {:#?}",
                move_number + 1, solution_move, current_state
            )
        }
    }

    if !current_state.is_solved() {
        panic!("Following the solution should lead to the solved state, but instead leads to {:#?}", current_state)
    }
}
