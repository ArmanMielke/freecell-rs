use std::fmt::{Debug, Display, Formatter, Error};

use super::{Card, CardCollection, Suit, ACE};



//const FOUNDATION_MAX_SIZE: usize = 13;


/// A stack of cards of one suit, ordered from Ace upwards.
///
/// # Rules
///
/// TODO
///
/// # Examples
///
/// ```
/// // TODO
/// ```
pub type Foundation = Vec<Card>;


/// Four stacks of cards, where each stack contains only cards of one suit, going from Ace upwards.
///
/// # Rules
///
/// TODO
///
/// # Usage
///
/// The position of the Foundation in the array determines which suit it holds.
/// Eg. `foundations[1]` may only hold spade cards, since Suit::Spade equals 1.
/// TODO explain more thoroughly
///
/// # Examples
///
/// ```
/// // TODO
/// ```
// TODO #[derive(Default)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Foundations(pub [Foundation; 4]);


impl Foundations {

    /// Creates empty foundations
    pub fn new() -> Foundations {
        Foundations([Vec::new(), Vec::new(), Vec::new(), Vec::new()])
    }

    /// Returns the foundation of the given suit
    pub fn foundation(&self, suit: Suit) -> &Foundation {
        &self.0[suit as usize]
    }

}


impl CardCollection for Foundations {

    fn add_card(&self, card: Card) -> Result<Self, ()> {
        // check whether the card can be put on any foundation
        if self.foundation(card.suit).is_empty() {
            // only Aces can be put on an empty foundation
            if card.rank != ACE {
                return Err(())
            }
        } else if
            // Aces can only be put on an empty foundation
            card.rank == ACE ||
            // Other cards can only be put on a foundation if it is one rank higher than the
            // currently topmost card on the foundation
            self.foundation(card.suit).last().unwrap().rank + 1 != card.rank
        {
            return Err(())
        }

        let mut clone = self.clone();
        clone.0[card.suit as usize].push(card);
        Ok(clone)
    }

    /// Always returns an empty Vec, since cards cannot be removed from foundations
    fn pop_card(&self) -> Vec<(Self, Card)> {
        Vec::new()
    }

}


impl Display for Foundations {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let foundation_strings: Vec<String> = self.0.iter().map(
            |foundation|
                if foundation.is_empty() {
                    "Empty".to_string()
                } else {
                    format!("Up to {}", foundation.last().unwrap().to_string())
                }
        ).collect();
        write!(f, "Foundations: {}", foundation_strings.join(", "))
    }
}


impl Debug for Foundations {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Foundations:")?;
        for foundation in &self.0 {
            if !foundation.is_empty() {
                write!(f, " {:?}", foundation.last().unwrap())?;
            }
        }
        Ok(())
    }
}
