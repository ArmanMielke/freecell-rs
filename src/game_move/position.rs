use std::fmt::{Display, Formatter, Result};

/// Indicates the position of a card.
///
/// The cascades are numbered from 0 to 7.
// TODO [v2+] need a normal version with Freecells(usize) and an efficient version with just Freecells
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Position {
    #[allow(missing_docs)]
    Cascade(usize),
    #[allow(missing_docs)]
    Foundations,
    #[allow(missing_docs)]
    Freecells,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Position::Cascade(pos) => write!(f, "Cascade {}", pos + 1),
            Position::Foundations => write!(f, "the Foundations"),
            Position::Freecells => write!(f, "the Freecells"),
        }
    }
}
