use freecell::{Card, Cascade, Foundation};
use freecell::card::Suit;


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
        Err("Too many cascades".to_string())
    } else {
        panic!("case cascades.len() < 8 not implemented yet")
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
            Foundation {
                suit: Suit::Club,
                cards: Vec::new(),
            },
            Foundation {
                suit: Suit::Spade,
                cards: Vec::new(),
            },
            Foundation {
                suit: Suit::Heart,
                cards: Vec::new(),
            },
            Foundation {
                suit: Suit::Diamond,
                cards: Vec::new(),
            },
        ])
    } else if foundations.len() > 4 {
        Err("Too many foundations".to_string())
    } else {
        panic!("case 0 < foundations.len() < 4 not implemented yet")
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
        Err("Too many freecells".to_string())
    } else {
        panic!("case 0 < freecells.len() < 4 not implemented yet")
    }
}
