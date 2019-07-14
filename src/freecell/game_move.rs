use super::Card;
use super::position::Position;

use std::fmt::{Display, Debug, Formatter, Result};



#[derive(PartialEq, Clone)]
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


#[cfg(test)]
impl Debug for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Move {:?} from {} to {}", self.card, self.from, self.to)
    }
}
