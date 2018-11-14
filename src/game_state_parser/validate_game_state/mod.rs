mod check_cards;
mod check_foundations;


use freecell::GameState;



/// Checks whether a game state is correct, i.e.
/// - each card exists exactly once
/// - each card on a foundation is of the correct suit
/// - the cards on a foundation are ordered correctly
pub fn validate_game_state(game_state: &GameState) -> Result<(), String> {
    check_cards::check_cards(game_state)?;
    check_foundations::check_foundations(game_state)?;
    Ok(())
}
