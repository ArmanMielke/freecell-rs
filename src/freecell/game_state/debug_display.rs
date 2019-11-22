use std::fmt::{Debug, Display, Formatter, Result};

use super::GameState;



impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // foundations
        writeln!(f, "{}\n", self.foundations)?;

        // cascades
        for (i, cascade) in self.cascades.iter().enumerate() {
            let cascade_cards: Vec<String> = cascade.iter().map(
                |card| card.to_string()
            ).collect();
            writeln!(f, "Cascade {}: {}", i + 1, cascade_cards.join(", "))?;
        }
        writeln!(f)?;

        // freecells
        let freecell_cards: Vec<String> = self.freecells.iter().map(
            |card| card.to_string()
        ).collect();
        writeln!(f, "Freecells: {}", freecell_cards.join(", "))
    }
}


// TODO [high priority] document that the debug output can be used as input file
// TODO [med priority] test whether this outputs in the same format that the test inputs use
impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // foundations
        writeln!(f, "{:?}\n", self.foundations)?;

        // cascades
        for cascade in &self.cascades {
            write!(f, "Cascade:")?;
            for card in cascade {
                write!(f, " {:?}", card)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        // freecells
        write!(f, "Freecells:")?;
        for card in &self.freecells {
            write!(f, " {:?}", card)?;
        }
        writeln!(f)
    }
}
