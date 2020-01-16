use std::str::FromStr;

use super::GameState;
use crate::{parse_cascade, parse_freecells, Cascade, Cascades, Foundations};

impl FromStr for GameState {
    type Err = String;

    // TODO [v1] document
    fn from_str(string: &str) -> Result<GameState, Self::Err> {
        let mut cascades: Vec<Cascade> = Vec::with_capacity(8);
        let mut foundations = Foundations::new();
        let mut freecells = [None, None, None, None];

        for line in string.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            let colon_position = match line.find(":") {
                Some(position) => position,
                None => return Err(format!("Could not parse line: \"{}\"", line)),
            };

            // the part before the colon determines how the part after the colon should be interpreted
            // TODO [med priority] make it so all parse functions can handle the first token, too, so that `after_colon` can be replaced by `line`
            let after_colon = &line[colon_position+1..];
            match line[0..colon_position].to_lowercase().trim() {
                "cascade" => cascades.push(parse_cascade(after_colon)?),
                "foundations" => foundations = line.parse()?,
                "freecells" => freecells = parse_freecells(after_colon)?,
                token => return Err(format!("Line starts with invalid token: \"{}\"", token)),
            };
        }

        let cascades = cascades_vec_to_array(cascades)?;
        let game_state = GameState { cascades, foundations, freecells };
        game_state.validate()?;
        Ok(game_state)
    }
}

fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<Cascades, String> {
    // make sure cascades has length 8
    if cascades.len() > 8 {
        // TODO [low priority] make the error message more informative by giving the maximum and actual numbers of cascades
        return Err("Too many cascades".to_string());
    }
    cascades.resize(8, Vec::new());

    // convert to array
    Ok([
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
    ])
}
