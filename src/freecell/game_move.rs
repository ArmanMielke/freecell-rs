use std::fmt::{Display, Formatter, Result};
#[cfg(test)]
use std::fmt::Debug;

use super::Card;
use super::position::Position;



#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
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
