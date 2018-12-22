use super::{ACE, JACK, KING, QUEEN};
use super::Card;
use super::Suit::*;

use std::fmt::{Debug, Formatter, Result};



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
