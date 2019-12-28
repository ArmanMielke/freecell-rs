#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use super::{Color, Colour};

/// Indicates the suit of a card.
///
/// Suits can be parsed from Strings or `&str`s.
/// The formats for this are the same formats used by Display and Debug.
/// See the description for `FromStr` below for details.
///
/// # Examples
///
/// Parsing suits from strings:
/// ```
/// # use freecell::Suit;
/// assert_eq!(Ok(Suit::Club), "Club".parse());
/// assert_eq!(Ok(Suit::Club), "Clubs".parse());
/// assert_eq!(Ok(Suit::Spade), "spade".parse());
/// assert_eq!(Ok(Suit::Heart), "H".parse());
/// assert_eq!(Ok(Suit::Diamond), "d".parse());
/// ```
///
/// A formatted suit can be converted back to the original suit:
/// ```
/// # use freecell::Suit;
/// // Formatted using Display
/// assert_eq!(Ok(Suit::Club), Suit::Club.to_string().parse());
/// // Formatted using Debug
/// assert_eq!(Ok(Suit::Club), format!("{:?}", Suit::Club).parse());
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Suit {
    #[allow(missing_docs)]
    Club = 0,
    #[allow(missing_docs)]
    Spade = 1,
    #[allow(missing_docs)]
    Heart = 2,
    #[allow(missing_docs)]
    Diamond = 3,
}

impl Suit {
    /// Returns the colour of this suit.
    ///
    /// Club and Spade are black, Heart and Diamond are red.
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell::Colour;
    /// # use freecell::Suit;
    ///
    /// assert_eq!(Colour::Black, Suit::Club.colour());
    /// assert_eq!(Colour::Red, Suit::Heart.colour());
    /// ```
    pub fn colour(self) -> Colour {
        match self {
            Suit::Club => Colour::Black,
            Suit::Spade => Colour::Black,
            Suit::Heart => Colour::Red,
            Suit::Diamond => Colour::Red,
        }
    }

    /// Returns the color of this suit.
    ///
    /// This is an alias of `colour()`.
    pub fn color(self) -> Color {
        self.colour()
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Suit::Club => "Club",
            Suit::Spade => "Spade",
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
        };
        write!(f, "{}", name)
    }
}

impl Debug for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Suit::Club => "C",
            Suit::Spade => "S",
            Suit::Heart => "H",
            Suit::Diamond => "D",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Suit {
    type Err = String;

    /// Converts a `&str` to a Suit.
    ///
    /// The string must be one of the following:
    /// "Club", "Clubs", "C",
    /// "Spade", "Spades", "S",
    /// "Heart", "Hearts", "H",
    /// "Diamond", "Diamonds" or "D".
    /// These strings are case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use freecell::Suit;
    /// assert_eq!(Ok(Suit::Club), "Club".parse());
    /// assert_eq!(Ok(Suit::Club), "Clubs".parse());
    /// assert_eq!(Ok(Suit::Club), "C".parse());
    /// assert_eq!(Ok(Suit::Heart), "heart".parse());
    /// assert_eq!(Ok(Suit::Heart), "hearts".parse());
    /// assert_eq!(Ok(Suit::Heart), "h".parse());
    /// ```
    fn from_str(string: &str) -> Result<Suit, Self::Err> {
        match string.to_lowercase().trim() {
            "club" => Ok(Suit::Club),
            "clubs" => Ok(Suit::Club),
            "c" => Ok(Suit::Club),
            "spade" => Ok(Suit::Spade),
            "spades" => Ok(Suit::Spade),
            "s" => Ok(Suit::Spade),
            "heart" => Ok(Suit::Heart),
            "hearts" => Ok(Suit::Heart),
            "h" => Ok(Suit::Heart),
            "diamond" => Ok(Suit::Diamond),
            "diamonds" => Ok(Suit::Diamond),
            "d" => Ok(Suit::Diamond),
            _ => Err(format!("Suit \"{}\" does not match any of \"C[lub[s]]\", \"S[pade[s]]\", \"H[eart[s]]\", \"D[iamond[s]]\" (case-insensitive)", string)),
        }
    }
}
