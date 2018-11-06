use freecell::{Card, Cascade, Foundation};
use freecell::card::Suit::*;
use super::error_messages::{ERR_TOO_MANY_CASCADES, ERR_TOO_MANY_FOUNDATIONS, ERR_TOO_MANY_FREECELLS};


pub fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<[Cascade; 8], String> {
    // TODO rewrite to handle cascades.len() < 8
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
    } else {
        unimplemented!()
    }
}

pub fn foundations_vec_to_array(mut foundations: Vec<Foundation>) -> Result<[Foundation; 4], String> {
    // it will be checked later whether all foundations have different suits
    // TODO rewrite to handle 0 < foundations.len() < 4
    if foundations.len() == 4 {
        Ok([
            foundations.remove(0),
            foundations.remove(0),
            foundations.remove(0),
            foundations.remove(0),
        ])
    } else if foundations.len() == 0 {
        Ok ([
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ])
    } else if foundations.len() > 4 {
        Err(String::from(ERR_TOO_MANY_FOUNDATIONS))
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
