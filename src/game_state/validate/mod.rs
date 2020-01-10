mod check_cards;
mod check_foundations;

use super::GameState;

impl GameState {
    /// Checks whether a game state is valid, i.e.
    /// - each of the 52 cards exists exactly once
    /// - each card on a foundation is of the correct suit
    /// - the cards on each foundation are ordered correctly
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO [v1] Add code examples (once FromStr is implemented for GameState)
    /// ```
    // TODO [med priority] needs more tests
    pub fn validate(&self) -> Result<(), String> {
        check_cards::check_cards(&self)?;
        check_foundations::check_foundations(&self)?;
        Ok(())
    }
}
