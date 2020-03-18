#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;

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
    #[allow(missing_docs)]
    pub cascades: Cascades,
    #[allow(missing_docs)]
    pub foundations: Foundations,
    #[allow(missing_docs)]
    pub freecells: Freecells,
}

impl GameState {
    /// Parses the contents of a file into a GameState.
    ///
    /// This is equivalent to reading the contents of the file as `&str` and using that as input
    /// for `GameState::from_str`
    // TODO consider borrowing the file name: `file_name: &P` it doesn't really need to be consumed here, right?
    pub fn from_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(_) => return Err("File could not be read".to_string()),
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Err("File contents could not be read".to_string())
        }

        contents.parse()
    }

    /// Returns `true` if all cards are on the foundations, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use freecell::GameState;
    /// assert!("foundations: KC KS KH KD".parse::<GameState>().unwrap().is_solved());
    /// assert!(
    ///     !"
    ///         foundations: KC KS KH QD
    ///         cascade: KD
    ///     ".parse::<GameState>().unwrap().is_solved()
    /// );
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
