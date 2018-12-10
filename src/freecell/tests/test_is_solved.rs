use game_state_parser::parse_file;



#[test]
fn test_solved() {
    let game_state = parse_file("test-inputs/solved.txt").unwrap();
    assert!(game_state.is_solved());
}

#[test]
fn test_easy() {
    let game_state = parse_file("test-inputs/easy.txt").unwrap();
    assert!(!game_state.is_solved());
}

#[test]
fn test_hard() {
    let game_state = parse_file("test-inputs/hard.txt").unwrap();
    assert!(!game_state.is_solved());
}

#[test]
fn test_hard_solved_to_6() {
    let game_state = parse_file("test-inputs/hard-solved-to-6.txt").unwrap();
    assert!(!game_state.is_solved());
}
