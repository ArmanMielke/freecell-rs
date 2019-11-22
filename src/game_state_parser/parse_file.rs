use arrayvec::ArrayVec;

use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str::SplitWhitespace;

use super::conversions_to_array;
use super::error_messages::{ERR_COULD_NOT_READ_FILE, ERR_COULD_NOT_READ_FILE_CONTENTS, ERR_TOO_MANY_FREECELLS};
use crate::freecell::{Card, Cascade, Foundations, Freecells, GameState};

// TODO [high priority] let all structs handle their own parsing (should be case-insensitive)

const FOUNDATIONS: &str = "foundations:";
const CASCADE: &str = "cascade:";
const FREECELLS: &str = "freecells:";

pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
    let lines = read_file_as_lines(file_name)?;

    let mut cascades: Vec<Cascade> = Vec::new();
    let mut foundations = Foundations::new();
    let mut freecells = ArrayVec::new();

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
            FOUNDATIONS => create_foundations(
                &mut foundations,
                parse_cards(token_iterator)?
            )?,
            CASCADE => cascades.push(parse_cards(token_iterator)?),
            FREECELLS => create_freecells(
                &mut freecells,
                parse_cards(token_iterator)?
            )?,
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

fn parse_cards(card_iterator: SplitWhitespace) -> Result<Vec<Card>, String> {
    let mut cards = Vec::new();

    for card_code in card_iterator {
        cards.push(Card::try_from(card_code)?);
    }

    Ok(cards)
}


fn create_foundations(foundations: &mut Foundations, foundation_cards: Vec<Card>) -> Result<(), String> {
    for card in foundation_cards  {
        if !foundations.foundation(card.suit).is_empty() {
            return Err(err_multiple_foundations_of_suit!(card.suit));
        } else {
            foundations.0[card.suit as usize] = card_sequence_up_to(card);
        }
    }

    Ok(())
}

fn card_sequence_up_to(card: Card) -> Vec<Card> {
    let mut cards = Vec::new();

    for rank in 1..=card.rank {
        cards.push(Card {
            suit: card.suit,
            rank,
        });
    }

    cards
}

fn create_freecells(freecells: &mut Freecells, freecell_cards: Vec<Card>) -> Result<(), String> {
    for card in freecell_cards {
        if freecells.try_push(card).is_err() {
            return Err(String::from(ERR_TOO_MANY_FREECELLS));
        }
    }

    Ok(())
}
