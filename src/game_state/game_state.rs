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

    // TODO [v1] document
    pub fn id(&self) -> GameStateId {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// TODO [v2+] add an alternative GameState struct where Eq is implemented so that the order of cascades and the order of cards in freecells don't matter.
