use crate::freecell::{Card, Rank, Suit, ACE, JACK, KING, QUEEN};
use crate::freecell::Suit::{Club, Diamond, Heart, Spade};



///
/// Parses a card code into a card.
///
/// A card code is a &str containing two characters.
/// The first character denotes the card's rank:
/// - 'A' -> Ace (alternate form '1' is also allowed)
/// - '2' -> 2
/// - ...
/// - '9' -> 9
/// - 'T' -> 10 (alternate form '0' is also allowed)
/// - 'J' -> Jack
/// - 'Q' -> Queen
/// - 'K' -> King
///
/// The second character denotes the suit:
/// - 'C' -> Club
/// - 'S' -> Spade
/// - 'H' -> Heart
/// - 'D' -> Diamond
///
pub fn parse_card(card_code: &str) -> Result<Card, String> {
    if card_code.chars().count() != 2 {
        return Err(err_card_code_not_length_2!(card_code))
    }

    let mut card_chars = card_code.chars();
    let rank_char = card_chars.next().unwrap();
    let suit_char = card_chars.next().unwrap();

    Ok(Card {
        suit: parse_suit(suit_char)?,
        rank: parse_rank(rank_char)?,
    })
}

fn parse_rank(rank_char: char) -> Result<Rank, String> {
    match rank_char {
        'A' => Ok(ACE),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'T' => Ok(10),
        'J' => Ok(JACK),
        'Q' => Ok(QUEEN),
        'K' => Ok(KING),
        // some alternate forms are allowed
        '1' => Ok(ACE),
        '0' => Ok(10),
        _  => Err(err_could_not_parse_card_rank!(rank_char))
    }
}

fn parse_suit(suit_char: char) -> Result<Suit, String> {
    match suit_char {
        'C' => Ok(Club),
        'S' => Ok(Spade),
        'H' => Ok(Heart),
        'D' => Ok(Diamond),
        _  => Err(err_could_not_parse_suit!(suit_char)),
    }
}
