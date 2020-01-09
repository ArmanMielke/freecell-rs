use lazy_static::lazy_static;
use regex::Regex;

use crate::card::CARD_PATTERN;
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

// TODO document
// TODO test
pub fn parse_freecells<S: Into<String>>(string: S) -> Result<Freecells, String> {
    lazy_static! {
        static ref FREECELLS_RE: Regex = Regex::new(format!(r"(?i)^\s*(({}|empty)\s*){}$", CARD_PATTERN, "{0,4}").as_str()).unwrap();
        static ref FREECELL_RE: Regex = Regex::new(format!(r"(?i){}|empty", CARD_PATTERN).as_str()).unwrap();
    }

    let string = &string.into();
    if !FREECELLS_RE.is_match(string) {
        return Err("Could not parse freecells".to_string())
    }

    let mut freecells = [None, None, None, None];

    for (i, re_match) in FREECELL_RE.find_iter(string).enumerate() {
        // if the match cannot be parsed into a card, then it was "empty" => leave this freecell at None
        if let Ok(card) = re_match.as_str().parse() {
            freecells[i] = Some(card);
        }
    }

    Ok(freecells)
}
