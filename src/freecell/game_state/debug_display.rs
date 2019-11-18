use std::fmt::{Debug, Display, Formatter, Result};

use super::GameState;



impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // foundations
        let foundation_strings: Vec<String> = self.foundations.iter().map(
            |foundation|
                if foundation.is_empty() {
                    "Empty".to_string()
                } else {
                    format!("Up to {}", foundation.last().unwrap().to_string())
                }
        ).collect();
        writeln!(f, "Foundations: {}", foundation_strings.join(", "))?;
        writeln!(f)?;

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


// TODO document that the debug output can be used as input file
// TODO test whether this outputs in the same format that the test inputs use
impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // foundations
        write!(f, "foundations:")?;
        for foundation in &self.foundations {
            if !foundation.is_empty() {
                write!(f, " {:?}", foundation.last().unwrap())?;
            }
        }
        writeln!(f)?;
        writeln!(f)?;

        // cascades
        for cascade in &self.cascades {
            write!(f, "cascade:")?;
            for card in cascade {
                write!(f, " {:?}", card)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        // freecells
        write!(f, "freecells:")?;
        for card in &self.freecells {
            write!(f, " {:?}", card)?;
        }
        writeln!(f)
    }
}
