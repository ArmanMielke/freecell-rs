
pub const JACK: u8 = 11;
pub const QUEEN: u8 = 12;
pub const KING: u8 = 13;
pub const ACE: u8 = 1;

pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

pub struct Rank(pub u8);

pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}
