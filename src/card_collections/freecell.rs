use crate::{Card, CardCollection};

/// May hold any card.
pub type Freecell = Option<Card>;

impl CardCollection for Freecell {
    fn add_card(&self, card: Card) -> Result<Self, ()> {
        match self {
            Some(_) => Err(()),
            None => Ok(Some(card)),
        }
    }

    fn pop_card(&self) -> Vec<(Self, Card)> {
        match self {
            Some(card) => vec![(None, *card)],
            None => Vec::with_capacity(0),
        }
    }
}

/// May hold up to four arbitrary cards.
pub type Freecells = [Freecell; 4];
