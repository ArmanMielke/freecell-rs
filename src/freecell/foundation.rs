use super::{Card, CardCollection};
use super::card::{ACE, Suit};

//const FOUNDATION_MAX_SIZE: usize = 13;



/// May only contain cards of one suit, ordered from Ace upwards.
// TODO does this need to be public?
pub type Foundation = Vec<Card>;

/// The position of the Foundation in the array determines which suit it holds.
/// Eg. `foundations[1]` may only hold spade cards, since Suit::Spade equals 1.
pub type Foundations = [Foundation; 4];


// TODO Use the trait everywhere where the type Foundations is used. Rename the trait to Foundations and rename the type to something else.
// TODO This includes implementing CardCollection for the Foundations trait. See https://stackoverflow.com/questions/31082179/is-there-a-way-to-implement-a-trait-on-top-of-another-trait
pub trait FoundationsTrait {
    // TODO rename to foundation()
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
        // check whether the card can be put on any foundation
        if self.get_foundation(card.suit).is_empty() {
            // only Aces can be put on an empty foundation
            if card.value != ACE {
                return Err(())
            }
        } else if
            // Aces can only be put on an empty foundation
            card.value == ACE ||
            // Other cards can only be put on a foundation if it is one rank higher than the
            // currently topmost card on the foundation
            self.get_foundation(card.suit).last().unwrap().value + 1 != card.value
        {
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
