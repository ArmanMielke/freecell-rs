
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;
pub const ACE: Rank = 1;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Club = 0,
    Spade = 1,
    Heart = 2,
    Diamond = 3,
}

pub type Rank = u8;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}
