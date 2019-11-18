use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::super::{Cascades, Foundations, Freecells};



pub type GameStateId = u64;


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GameState {
    pub cascades: Cascades,
    pub foundations: Foundations,
    pub freecells: Freecells,
}

impl GameState {

    pub fn is_solved(&self) -> bool {
        self.foundations[0].len() == 13 &&
        self.foundations[1].len() == 13 &&
        self.foundations[2].len() == 13 &&
        self.foundations[3].len() == 13
    }

    pub fn id(&self) -> GameStateId {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

}

// TODO implement Eq. Order of cascades and order of cards in freecells should not matter.
