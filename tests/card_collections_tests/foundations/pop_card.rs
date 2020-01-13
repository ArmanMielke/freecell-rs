use freecell::{CardCollection, Foundations, GameState};

#[test]
fn test_empty() {
    let foundations = Foundations::new();
    assert_eq!(foundations.pop_card(), Vec::new());
}

#[test]
fn test_not_empty() {
    let game_state = GameState::from_file("test-inputs/hard-solved-to-4.txt").unwrap();
    assert_eq!(game_state.foundations.pop_card(), Vec::new());
}
