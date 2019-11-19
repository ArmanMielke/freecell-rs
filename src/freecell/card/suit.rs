use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Debug, Formatter};

use super::Colour;



/// Indicates the suit of a card.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Suit {
    Club = 0,
    Spade = 1,
    Heart = 2,
    Diamond = 3,
}


impl Suit {
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

    fn try_from(string: String) -> Result<Suit, Self::Error> {
        match string.trim().to_lowercase().as_ref() {
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
