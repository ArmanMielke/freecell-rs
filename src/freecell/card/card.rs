use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::{Rank, Suit, ACE, JACK, KING, QUEEN};



/// Represents one card in the game.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}


impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rank_string = match self.rank {
            JACK => String::from("Jack"),
            QUEEN => String::from("Queen"),
            KING => String::from("King"),
            ACE => String::from("Ace"),
            _ => format!("{}", self.rank),
        };
        write!(f, "{} of {}s", rank_string, self.suit)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rank_string = match self.rank {
            JACK => String::from("J"),
            QUEEN => String::from("Q"),
            KING => String::from("K"),
            ACE => String::from("A"),
            10 => String::from("T"),
            _ => format!("{}", self.rank),
        };

        write!(f, "{}{:?}", rank_string, self.suit)
    }
}
