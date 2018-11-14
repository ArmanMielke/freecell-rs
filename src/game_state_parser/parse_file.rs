use freecell::{Card, Cascade, Foundation, GameState};
use super::parse_card::parse_card;
use super::conversions_to_array::*;
use super::error_messages::{ERR_COULD_NOT_READ_FILE, ERR_COULD_NOT_READ_FILE_CONTENTS};

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::str::SplitWhitespace;

const FOUNDATIONS: &str = "foundations:";
const CASCADE: &str = "cascade:";
const FREECELLS: &str = "freecells:";


// TODO add documentation (use the template explanation in doc/)
pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
    let lines = read_file_as_lines(file_name)?;

    let mut cascades: Vec<Cascade> = Vec::new();
    // TODO use constant NUM_FOUNDATIONS instead of a literal 4
    let mut foundations: [Foundation; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut freecells: Vec<Card> = Vec::new();

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
            FOUNDATIONS => create_foundations(&mut foundations, parse_cards(token_iterator)?)?,
            CASCADE => cascades.push(parse_cards(token_iterator)?),
            FREECELLS => freecells = parse_cards(token_iterator)?,
            _ => warn_invalid_first_token!(first_token_in_line),
        };
    }


    Ok(GameState {
        cascades: cascades_vec_to_array(cascades)?,
        foundations,
        freecells: freecells_vec_to_array(freecells)?,
    })
}


fn read_file_as_lines<P: AsRef<Path>>(file_name: P) -> Result<Lines<BufReader<File>>, String> {
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
        cards.push(parse_card(card_code)?);
    }

    Ok(cards)
}


// TODO use constant NUM_FOUNDATIONS instead of a literal 4
fn create_foundations(foundations: &mut [Foundation; 4], foundation_cards: Vec<Card>) -> Result<(), String> {
    for foundation_card in foundation_cards  {
        if foundations[foundation_card.suit as usize].len() > 0 {
            return Err(err_multiple_foundations_of_suit!(foundation_card.suit))
        } else {
            foundations[foundation_card.suit as usize] = card_sequence_up_to(&foundation_card);
        }
    }

    Ok(())
}

fn card_sequence_up_to(card: &Card) -> Vec<Card> {
    let mut cards = Vec::new();

    for rank in 0..card.value {
        cards.push(Card{
            suit: card.suit,
            value: rank,
        });
    }

    cards
}
