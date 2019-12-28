#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Formatter, Result};

/// All possible positions.
pub const POSITIONS: [Position; 13] = [
    Position::Cascade(0),
    Position::Cascade(1),
    Position::Cascade(2),
    Position::Cascade(3),
    Position::Cascade(4),
    Position::Cascade(5),
    Position::Cascade(6),
    Position::Cascade(7),
    Position::Foundations,
    Position::Freecell(0),
    Position::Freecell(1),
    Position::Freecell(2),
    Position::Freecell(3),
];

/// Indicates the position of a card.
///
/// The cascades are numbered from 0 to 7.
// TODO [v2+] need a normal version with Freecells(usize) and an efficient version with just Freecells
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Position {
    #[allow(missing_docs)]
    Cascade(usize),
    #[allow(missing_docs)]
    Foundations,
    #[allow(missing_docs)]
    Freecell(usize),
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Position::Cascade(i) => write!(f, "Cascade {}", i + 1),
            Position::Foundations => write!(f, "the Foundations"),
            Position::Freecell(i) => write!(f, "Freecell {}", i + 1),
        }
    }
}
