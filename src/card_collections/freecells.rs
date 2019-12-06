use arrayvec::ArrayVec;

use crate::{Card, CardCollection};

/// May hold up to four arbitrary cards.
// TODO [v2+] create an optimised version of Freecells, where the order of cards doesn't matter
// data structures to consider:
// - HashSet, BTreeSet
// - https://lib.rs/crates/map_vec
// - https://lib.rs/crates/uset
// need to test performance for all of those
pub type Freecells = ArrayVec<[Card; 4]>;

impl CardCollection for Freecells {
    fn add_card(&self, card: Card) -> Result<Self, ()> {
        if self.is_full() {
            return Err(());
        }

        let mut clone = self.clone();
        clone.push(card);
        Ok(clone)
    }

    fn pop_card(&self) -> Vec<(Self, Card)> {
        let mut results = Vec::new();

        // for each card, return a result where this card has been removed
        for i in 0..self.len() {
            let mut clone = self.clone();
            let card = clone.swap_remove(i);
            results.push((clone, card))
        }

        results
    }
}
