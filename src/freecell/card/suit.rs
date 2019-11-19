use std::fmt::{Display, Formatter, Result};

use super::Colour;



#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = match self {
            Suit::Club => "Club",
            Suit::Spade => "Spade",
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
        };
        write!(f, "{}", name)
    }
}
