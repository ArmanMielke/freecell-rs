mod check_cards;
mod check_foundations;

use super::GameState;

impl GameState {
    /// Checks whether a game state is correct, i.e.
    /// - each card exists exactly once
    /// - each card on a foundation is of the correct suit
    /// - the cards on a foundation are ordered correctly
    // TODO [med priority] needs more tests
    pub fn validate(&self) -> Result<(), String> {
        check_cards::check_cards(&self)?;
        check_foundations::check_foundations(&self)?;
        Ok(())
    }
}
