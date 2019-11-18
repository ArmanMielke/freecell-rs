use std::fmt::{Debug, Display, Formatter, Result};

use super::Card;
use super::position::Position;



/// Represents one step in the game, where a card is moved from one position to another.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Move {
    pub card: Card,
    pub from: Position,
    pub to: Position,
}


impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Move the {} from {} to {}", self.card, self.from, self.to)
    }
}


impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Move {:?} from {} to {}", self.card, self.from, self.to)
    }
}
