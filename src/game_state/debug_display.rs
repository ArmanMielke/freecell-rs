use std::fmt::{Debug, Display, Formatter, Result};

use super::GameState;

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // foundations
        writeln!(f, "{}\n", self.foundations)?;

        // cascades
        for (i, cascade) in self.cascades.iter().enumerate() {
            let cascade_cards: Vec<String> = cascade.0.iter().map(
                |card| card.to_string()
            ).collect();
            writeln!(f, "Cascade {}: {}", i + 1, cascade_cards.join(", "))?;
        }
        writeln!(f)?;

        // freecells
        let freecell_cards: Vec<String> = self.freecells.iter().map(
            |some_card| match some_card {
                Some(card) => card.to_string(),
                None => String::from("Empty")
            }
        ).collect();
        writeln!(f, "Freecells: {}", freecell_cards.join(", "))
    }
}

// TODO [v1] document that the Debug output can be used as input for FromStr
// TODO [v1] make it so the same is true for Display (and document that)
// TODO [med priority] test whether this outputs in the same format that the test inputs use
impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // foundations
        writeln!(f, "{:?}\n", self.foundations)?;

        // cascades
        for cascade in &self.cascades {
            write!(f, "Cascade:")?;
            for card in cascade.0.iter() {
                write!(f, " {:?}", card)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        // freecells
        write!(f, "Freecells:")?;
        for some_card in &self.freecells {
            match some_card {
                Some(card) => write!(f, " {:?}", card)?,
                None => write!(f, " Empty")?,
            };
        }
        writeln!(f)
    }
}
