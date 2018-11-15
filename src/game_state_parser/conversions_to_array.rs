use freecell::{Card, Cascade};
use super::error_messages::{ERR_TOO_MANY_CASCADES, ERR_TOO_MANY_FREECELLS};


pub fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<[Cascade; 8], String> {
    // TODO implement this properly
    if cascades.len() == 8 {
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
    } else if cascades.len() > 8 {
        Err(String::from(ERR_TOO_MANY_CASCADES))
    } else if cascades.len() == 7 {
        Ok([
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            Vec::new(),
        ])
    } else {
        unimplemented!()
    }
}


pub fn freecells_vec_to_array(mut freecells: Vec<Card>) -> Result<[Option<Card>; 4], String> {
    // TODO rewrite to handle 0 < freecells.len() < 4
    if freecells.len() == 4 {
        Ok([
            Some(freecells.remove(0)),
            Some(freecells.remove(0)),
            Some(freecells.remove(0)),
            Some(freecells.remove(0)),
        ])
    } else if freecells.len() == 0 {
        Ok([None, None, None, None])
    } else if freecells.len() > 4 {
        Err(String::from(ERR_TOO_MANY_FREECELLS))
    } else {
        unimplemented!()
    }
}
