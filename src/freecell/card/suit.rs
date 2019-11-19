use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Debug, Formatter};

use super::Colour;



/// Indicates the suit of a card.
///
/// Suits can be converted from Strings.
/// The formats for this are the same formats used by Display and Debug.
///
/// # Examples
///
/// ```
/// # use std::convert::TryFrom;
/// # use freecell::Suit;
/// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("Club")));
/// assert_eq!(Ok(Suit::Spade), Suit::try_from(String::from("spade")));
/// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("H")));
/// assert_eq!(Ok(Suit::Diamond), Suit::try_from(String::from("d")));
///
/// // The formatted suit can be converted back to the original suit
/// assert_eq!(
///     Ok(Suit::Club),
///     Suit::try_from(Suit::Club.to_string())  // format using Display
/// );
/// assert_eq!(
///     Ok(Suit::Club),
///     Suit::try_from(format!("{:?}", Suit::Club))  // format using Debug
/// );
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Suit {
    Club = 0,
    Spade = 1,
    Heart = 2,
    Diamond = 3,
}


impl Suit {
    /// Returns the colour of this suit.
    ///
    /// Club and Spade are black, Heart and Diamond are red.
    pub fn colour(&self) -> Colour {
        match self {
            Suit::Club => Colour::Black,
            Suit::Spade => Colour::Black,
            Suit::Heart => Colour::Red,
            Suit::Diamond => Colour::Red,
        }
    }
}


impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Suit::Club => "C",
            Suit::Spade => "S",
            Suit::Heart => "H",
            Suit::Diamond => "D",
        };
        write!(f, "{}", name)
    }
}


impl TryFrom<String> for Suit {
    type Error = &'static str;

    /// Converts a String to a Suit.
    ///
    /// The string must be one of the following:
    /// "Club", "C", "Spade", "S", "Heart", "H", "Diamond" or "D".
    /// These strings are case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use freecell::Suit;
    /// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("Club")));
    /// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("C")));
    /// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("heart")));
    /// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("h")));
    /// ```
    fn try_from(string: String) -> Result<Suit, Self::Error> {
        match string.to_lowercase().trim() {
            "club" => Ok(Suit::Club),
            "c" => Ok(Suit::Club),
            "spade" => Ok(Suit::Spade),
            "s" => Ok(Suit::Spade),
            "heart" => Ok(Suit::Heart),
            "h" => Ok(Suit::Heart),
            "diamond" => Ok(Suit::Diamond),
            "d" => Ok(Suit::Diamond),
            _ => Err("Only the following values are accepted: \"Club\", \"C\", \"Spade\", \"S\", \"Heart\", \"H\", \"Diamond\" and \"D\" (case-insensitive)"),
        }
    }
}
