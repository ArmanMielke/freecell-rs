
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;
pub const ACE: Rank = 1;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

pub type Rank = u8;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}
