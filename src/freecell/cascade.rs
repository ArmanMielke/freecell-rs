use super::{Card, CardCollection};
use super::card::ACE;



//const CASCADE_MAX_SIZE: usize = 52;


/// A stack of arbitrary cards.
/// May only be used as a stack (TODO: enforce this).
pub type Cascade = Vec<Card>;


fn fits_on_top_of(lower_card: &Card, higher_card: &Card) -> bool {
    lower_card.suit.colour() != higher_card.suit.colour() &&
    lower_card.value + 1 == higher_card.value
}

impl CardCollection for Cascade {
    fn add_card(&self, card: Card) -> Result<Cascade, ()> {
        // optimisation: aces cannot be put on cascades
        // this is technically a legal move, just not a very good one
        if card.value == ACE {
            return Err(())
        }

        match self.last() {

            // the cascade contains at least one card
            Some(top_card) => {
                if fits_on_top_of(&card, &top_card) {
                    // the new card can be put onto this cascade
                    let mut clone = (*self).clone();
                    clone.push(card);
                    Ok(clone)
                } else {
                    // the new card cannot be put onto this cascade
                    Err(())
                }
            },

            // the cascade is empty => the card can be put here, creating a cascade with one card
            None => Ok(vec![card]),
        }
    }

    fn pop_card(&self) -> Vec<(Cascade, Card)> {
        let mut clone = (*self).clone();
        match clone.pop() {
            Some(card) => vec![(clone, card)],
            None => Vec::new(),
        }
    }
}


pub type Cascades = [Cascade; 8];
