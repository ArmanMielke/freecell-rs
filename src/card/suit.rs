use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::{Color, Colour};

/// Indicates the suit of a card.
///
/// Suits can be converted from Strings or `&str`s.
/// The formats for this are the same formats used by Display and Debug.
/// See the descriptions for `TryFrom<String>` and `TryFrom<&str>` below for details.
///
/// # Examples
///
/// Parsing suits from strings:
/// ```
/// use std::convert::TryFrom;
/// # use freecell::Suit;
///
/// assert_eq!(Ok(Suit::Club), Suit::try_from("Club"));
/// assert_eq!(Ok(Suit::Club), Suit::try_from("Clubs"));
/// assert_eq!(Ok(Suit::Spade), Suit::try_from("spade"));
/// assert_eq!(Ok(Suit::Heart), Suit::try_from("H"));
/// assert_eq!(Ok(Suit::Diamond), Suit::try_from("d"));
/// ```
///
/// A formatted suit can be converted back to the original suit:
/// ```
/// # use std::convert::TryFrom;
/// # use freecell::Suit;
/// // Formatted using Display
/// assert_eq!(Ok(Suit::Club), Suit::try_from(Suit::Club.to_string()));
/// // Formatted using Debug
/// assert_eq!(Ok(Suit::Club), Suit::try_from(format!("{:?}", Suit::Club)));
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

// TODO [low priority] is there a solution for this:
// would like this to be
// impl<S: Into<String>> TryFrom<S> for Suit
// but this doesn't work because of https://github.com/rust-lang/rust/issues/50133
impl TryFrom<String> for Suit {
    type Error = String;

    /// Converts a String to a Suit.
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
    /// # use std::convert::TryFrom;
    /// # use freecell::Suit;
    /// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("Club")));
    /// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("Clubs")));
    /// assert_eq!(Ok(Suit::Club), Suit::try_from(String::from("C")));
    /// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("heart")));
    /// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("hearts")));
    /// assert_eq!(Ok(Suit::Heart), Suit::try_from(String::from("h")));
    /// ```
    fn try_from(string: String) -> Result<Suit, Self::Error> {
        // TODO [low priority] use regex
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

impl TryFrom<&str> for Suit {
    type Error = String;

    /// Converts a `&str` to a Suit.
    ///
    /// See the description of `TryFrom<String>` for details.
    fn try_from(string: &str) -> Result<Suit, Self::Error> {
        Suit::try_from(string.to_string())
    }
}
