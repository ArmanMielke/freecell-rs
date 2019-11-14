use game_state_parser::parse_file;
use solve;
use super::check_solution::check_solution;



#[test]
fn test_hard_solved_to_q() {
    let initial_state = parse_file("test-inputs/hard-solved-to-Q.txt").unwrap();
    let solution = solve(initial_state.clone()).unwrap();
    check_solution(solution, initial_state, 4);
}
