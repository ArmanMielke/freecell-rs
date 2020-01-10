#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{Cascades, Foundations, Freecells};

pub type GameStateId = u64;

/// Represents the state of the board at one point during the course of a game.
///
/// The game state is defined by the cards that are in each of the three parts of the board, the
/// cascades, the foundations and the freecells.
///
/// A game state is valid if these conditions hold:
/// - Each of the 52 cards exists exactly once
/// - Each card on a foundation is of the correct suit
/// - The cards on each foundation are ordered correctly
///
/// # Examples
///
/// ```
/// // TODO [v1] Add code examples (once FromStr is implemented for GameState)
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameState {
    pub cascades: Cascades,
    pub foundations: Foundations,
    pub freecells: Freecells,
}

impl GameState {
    /// Returns `true` if all cards are on the foundations, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO [v1] Add code examples (once FromStr is implemented for GameState)
    /// ```
    pub fn is_solved(&self) -> bool {
        self.foundations.0[0].len() == 13 &&
        self.foundations.0[1].len() == 13 &&
        self.foundations.0[2].len() == 13 &&
        self.foundations.0[3].len() == 13
    }

    // TODO [v1] document
    pub fn id(&self) -> GameStateId {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
