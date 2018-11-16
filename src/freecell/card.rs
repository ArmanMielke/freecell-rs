

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
