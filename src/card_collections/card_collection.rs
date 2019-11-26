use std::marker::Sized;

use crate::Card;

pub trait CardCollection
where
    Self: Sized,
{
    /// Creates a copy of the CardCollection, but with an additional card added.
    /// Fails if the card cannot be put into the collection.
    fn add_card(&self, card: Card) -> Result<Self, ()>;

    /// Creates all possible versions of the CardCollection where one card has been removed.
    /// Each returned CardCollection is paired with the removed card.
    /// If no card can be removed according to the rules, an empty Vec is returned.
    fn pop_card(&self) -> Vec<(Self, Card)>;
}
