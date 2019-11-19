use std::fmt::{Debug, Display, Formatter, Result};

use super::{Rank, Suit, ACE, JACK, KING, QUEEN};



/// Represents one card in the game.
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

        write!(f, "{}{:?}", value_string, self.suit)
    }
}
