use crate::game_state_parser::parse_file;
use crate::solve;
use super::check_solution::check_solution;

use std::path::Path;



fn solve_and_check_file<P: AsRef<Path>>(file_name: P, correct_length: usize) {
    let initial_state = parse_file(file_name).unwrap();
    let solution = solve(initial_state.clone()).unwrap();
    check_solution(solution, initial_state, correct_length);
}

#[test]
fn test_hard_solved_to_q() {
    solve_and_check_file("test-inputs/hard-solved-to-Q.txt", 4)
}

#[test]
#[ignore]
// TODO solving this is infeasible with the current solver (>15 minutes on a weak machine)
fn test_hard_solved_to_j() {
    solve_and_check_file("test-inputs/hard-solved-to-J.txt", 9)
}
