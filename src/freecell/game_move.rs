use super::Card;
use super::position::Position;

use std::fmt::{Display, Formatter, Result};



pub struct Move {
    pub card: Card,
    pub from: Position,
    pub to: Position,
}


impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Move the {}, from {} to {}", self.card, self.from, self.to)
    }
}
