use freecell::{GameState, Move};



pub fn check_solution(solution: Vec<Move>, initial_state: GameState, correct_length: usize) {
    check_solution_length(&solution, correct_length);
    check_solution_correctness(solution, initial_state)
}

fn check_solution_length(solution: &Vec<Move>, correct_length: usize) {
    if solution.len() != correct_length {
        panic!("Solution has length {}, should have length {}", solution.len(), correct_length);
    }
}

fn check_solution_correctness(solution: Vec<Move>, initial_state: GameState) {
    // TODO implement
}
