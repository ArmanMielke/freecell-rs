#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt::{Debug, Display, Formatter, Result};

use crate::{Card, Position};

/// Represents one step in the game, where a card is moved from one position to another.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Move {
    #[allow(missing_docs)]
    pub card: Card,
    #[allow(missing_docs)]
    pub from: Position,
    #[allow(missing_docs)]
    pub to: Position,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Move the {} from {} to {}", self.card, self.from, self.to)
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Move {:?} from {} to {}", self.card, self.from, self.to)
    }
}
