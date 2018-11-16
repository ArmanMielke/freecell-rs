use super::Card;
use super::card::Suit;

//const FOUNDATION_MAX_SIZE: usize = 13;



/// May only contain cards of one suit, ordered from Ace upwards (TODO: enforce this).
pub type Foundation = Vec<Card>;

/// The position of the Foundation in the array determines which suit it holds.
/// Eg. `foundations[1]` may only hold spade cards, since Suit::Spade equals 1.
pub type Foundations = [Foundation; 4];


pub trait FoundationsTrait {
    fn get_foundation(&self, suit: Suit) -> &Foundation;
}

impl FoundationsTrait for Foundations {
    /// returns the foundation of the given suit
    fn get_foundation(&self, suit: Suit) -> &Foundation {
        &self[suit as usize]
    }
}
