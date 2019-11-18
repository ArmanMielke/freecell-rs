use std::fmt::{Display, Formatter, Result};

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


// TODO implement Debug. It should output in the same format that game_state_parser uses.
