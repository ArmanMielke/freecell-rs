use super::card::{Card, Suit};

const FOUNDATION_MAX_SIZE: usize = 13;

/// Contains cards of one suit, ordered from Ace updwards
pub struct Foundation {
    /// All cards on this foundation must be of this suit
    pub suit: Suit,
    pub cards: Vec<Card>,
}
