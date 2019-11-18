use std::fmt::{Debug, Display, Formatter, Result};

use super::{Rank, Suit, ACE, JACK, KING, QUEEN};
use super::Suit::{Club, Diamond, Heart, Spade};



#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value_string = match self.value {
            JACK => String::from("J"),
            QUEEN => String::from("Q"),
            KING => String::from("K"),
            ACE => String::from("A"),
            10 => String::from("T"),
            _ => format!("{}", self.value),
        };

        let suit_string = match self.suit {
            Club => String::from("C"),
            Spade => String::from("S"),
            Heart => String::from("H"),
            Diamond => String::from("D"),
        };

        write!(f, "{}{}", value_string, suit_string)
    }
}
