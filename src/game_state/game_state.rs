use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{Cascades, Foundations, Freecells};

pub type GameStateId = u64;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GameState {
    pub cascades: Cascades,
    pub foundations: Foundations,
    pub freecells: Freecells,
}

impl GameState {
    /// Returns `true` if all cards are on the foundations, `false` otherwise.
    pub fn is_solved(&self) -> bool {
        self.foundations.0[0].len() == 13 &&
        self.foundations.0[1].len() == 13 &&
        self.foundations.0[2].len() == 13 &&
        self.foundations.0[3].len() == 13
    }

    // TODO document
    pub fn id(&self) -> GameStateId {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// TODO [high/med priority] implement Eq. Order of cascades and order of cards in freecells should not matter.
