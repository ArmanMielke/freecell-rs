use solve;



#[test]
fn test_hard_solved_to_q() {
    // TODO test more precisely
    let solution = solve("test-inputs/hard-solved-to-Q.txt").unwrap();
    assert_eq!(solution.len(), 4)
}
