use std::fmt::{Display, Formatter, Result};


#[derive(Debug, PartialEq, Copy, Clone)]
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


#[derive(PartialEq)]
pub enum Colour {
    Black, Red
}



pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;
pub const ACE: Rank = 1;

pub type Rank = u8;



#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value_string = match self.value {
            JACK => String::from("Jack"),
            QUEEN => String::from("Queen"),
            KING => String::from("King"),
            ACE => String::from("Ace"),
            _ => format!("{}", self.value),
        };
        write!(f, "{} of {}s", value_string, self.suit)
    }
}
