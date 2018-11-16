use super::{Card, CardCollection};
use super::card::{ACE, Suit};

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


impl CardCollection for Foundations {

    fn add_card(&self, card: Card) -> Result<Self, ()> {
            // Aces can only be put on empty foundations
        if  card.value == ACE && !self.get_foundation(card.suit).is_empty() ||
            // Other cards can only be put on a foundation if it is one rank higher than the
            // currently topmost card on the foundation
            self.get_foundation(card.suit).last().unwrap().value != card.value + 1 {
            return Err(())
        }

        let mut clone = self.clone();
        clone[card.suit as usize].push(card);
        Ok(clone)
    }

    /// Always returns an empty Vec, since cards cannot be removed from foundations
    fn pop_card(&self) -> Vec<(Self, Card)> {
        Vec::new()
    }

}
