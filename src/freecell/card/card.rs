use super::{Rank, Suit, ACE, JACK, KING, QUEEN};

use std::fmt::{Display, Formatter, Result};



#[derive(PartialEq, Copy, Clone, Hash)]
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
