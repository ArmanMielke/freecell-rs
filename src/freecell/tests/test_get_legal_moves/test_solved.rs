use game_state_parser::parse_file;



// no moves possible
#[test]
fn test_solved() {
    let game_state = parse_file("test-inputs/solved.txt").unwrap();
    let actual = game_state.get_legal_moves();
    assert!(actual.is_empty());
}
