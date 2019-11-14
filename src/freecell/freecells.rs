use super::{Card, CardCollection};

use arrayvec::ArrayVec;



/// May hold up to four arbitrary cards
pub type Freecells = ArrayVec<[Card; 4]>;


// TODO the order of the cards shouldn't matter, e.g. {QH, TC, 9D} and {9D, QH, TC} should be equivalent. Possible solutions: Use set-like data structure or always sort array
impl CardCollection for Freecells {

    fn add_card(&self, card: Card) -> Result<Self, ()> {
        if self.is_full() {
            return Err(())
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
