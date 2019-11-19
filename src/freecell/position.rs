use std::fmt::{Display, Formatter, Result};



/// Indicates the position of a card.
///
/// The cascades are numbered from 0 to 7.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Position {
    Cascade(usize),
    Foundations,
    Freecells
}


impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Position::Cascade(pos) => write!(f, "Cascade {}", pos + 1),
            Position::Foundations => write!(f, "the Foundations"),
            Position::Freecells => write!(f, "the Freecells"),
        }
    }
}
