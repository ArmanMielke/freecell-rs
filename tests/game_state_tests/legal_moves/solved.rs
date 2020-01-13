use freecell::GameState;

// no moves possible
#[test]
fn test_solved() {
    let game_state = GameState::from_file("test-inputs/solved.txt").unwrap();
    let actual = game_state.legal_moves();
    assert!(actual.is_empty());
}
