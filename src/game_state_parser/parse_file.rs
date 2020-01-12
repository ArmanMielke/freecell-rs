use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::SplitWhitespace;

use super::conversions_to_array;
use super::error_messages::{ERR_COULD_NOT_READ_FILE, ERR_COULD_NOT_READ_FILE_CONTENTS};
use crate::{parse_cascade, parse_freecells, Card, Cascade, Foundations, Freecells, GameState};

// TODO [v1] let all structs handle their own parsing (should be case-insensitive)

const FOUNDATIONS: &str = "foundations:";
const CASCADE: &str = "cascade:";
const FREECELLS: &str = "freecells:";

#[allow(missing_docs)]
pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
    let lines = read_file_as_lines(file_name)?;

    let mut cascades: Vec<Cascade> = Vec::with_capacity(8);
    let mut foundations = Foundations::new();
    let mut freecells: Freecells = [None, None, None, None];

    for line_result in lines {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => return Err(String::from(ERR_COULD_NOT_READ_FILE_CONTENTS)),
        };

        let mut token_iterator = line.split_whitespace();

        let first_token_in_line = match token_iterator.next() {
            Some(token) => token,
            None => continue,
        };

        match first_token_in_line {
            FOUNDATIONS => foundations = token_iterator.fold(
                String::new(),
                |mut string, token| {string.push_str(token); string.push(' '); string}
            ).parse()?,
            CASCADE => cascades.push(parse_cascade(token_iterator.fold(
                String::new(),
                |mut string, token| {string.push_str(token); string.push(' '); string}
            ))?),
            FREECELLS => freecells = parse_freecells(token_iterator.fold(
                String::new(),
                |mut string, token| {string.push_str(token); string.push(' '); string}
            ))?,
            _ => warn_invalid_first_token!(first_token_in_line),
        };
    }

    let game_state = GameState {
        cascades: conversions_to_array::cascades_vec_to_array(cascades)?,
        foundations,
        freecells,
    };

    game_state.validate()?;

    Ok(game_state)
}

fn read_file_as_lines<P: AsRef<Path>>(file_name: P) -> Result<Lines<BufReader<File>>, String> {
    // TODO [med priority] return a proper error if the file does not exist, along the lines of 'File "<file name>" does not exist' or 'File "<file name>" not found'
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => return Err(String::from(ERR_COULD_NOT_READ_FILE)),
    };

    let buffered_reader = BufReader::new(file);
    Ok(buffered_reader.lines())
}

fn parse_cards(card_iterator: SplitWhitespace<'_>) -> Result<Vec<Card>, String> {
    let mut cards = Vec::new();

    for card_code in card_iterator {
        cards.push(card_code.parse()?);
    }

    Ok(cards)
}
