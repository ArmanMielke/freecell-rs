
const JACK: u8 = 11;
const QUEEN: u8 = 12;
const KING: u8 = 13;
const ACE: u8 = 1;

pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

pub struct Rank(pub u8);

pub struct Card {
    suit: Suit,
    value: Rank,
}
