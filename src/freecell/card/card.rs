use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::{Rank, Suit, rank_from_string, ACE, JACK, KING, QUEEN};



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


// TODO is there a solution for this:
// would like this to be
// impl<S: Into<String>> TryFrom<S> for Card
// but this doesn't work because of https://github.com/rust-lang/rust/issues/50133
impl TryFrom<String> for Card {
    type Error = String;

    // TODO document
    fn try_from(string: String) -> Result<Card, Self::Error> {
        let string = string.trim();

        if string.len() == 2 {
            // string uses the short Debug format
            // the first character is the rank, the second character is the suit
            let suit = Suit::try_from(&string[1..2])?;
            let rank = rank_from_string(&string[0..1])?;
            Ok(Card { suit, rank })
        } else {
            // string seems to use the long Display format
            // the first word is the rank, the second word is the suit
            let string: Vec<&str> = string.split(' ').collect();
            let suit = Suit::try_from(*string.last().unwrap())?;
            let rank = rank_from_string(string[0])?;
            Ok(Card { suit, rank })
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    // TODO document
    fn try_from(string: &str) -> Result<Card, Self::Error> {
        return Card::try_from(string.to_string())
    }
}
