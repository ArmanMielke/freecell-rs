#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Formatter, Result};

/// Indicates the colour of a card.
///
/// The suits Club and Spade are black, whereas Heart and Diamond are red.
///
/// # Examples
///
/// ```
/// use freecell::{Card, Colour, Suit, ACE, KING};
///
/// let black_card = Card {suit: Suit::Spade, rank: ACE};
/// assert_eq!(Colour::Black, black_card.suit.colour());
///
/// let red_card = Card {suit: Suit::Heart, rank: KING};
/// assert_eq!(Colour::Red, red_card.suit.colour());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Colour {
    #[allow(missing_docs)]
    Black,
    #[allow(missing_docs)]
    Red,
}

/// Alias for the American spelling of Colour.
pub type Color = Colour;

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Colour::Black => write!(f, "black"),
            Colour::Red => write!(f, "red"),
        }
    }
}
