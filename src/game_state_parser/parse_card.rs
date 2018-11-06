use freecell::card::*;


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
    let value_char = card_chars.next().unwrap();
    let suit_char = card_chars.next().unwrap();

    Ok(Card {
        suit: parse_suit(suit_char)?,
        value: parse_value(value_char)?,
    })
}

fn parse_value(value_char: char) -> Result<Rank, String> {
    match value_char {
        'A' => Ok(Rank(ACE)),
        '2' => Ok(Rank(2)),
        '3' => Ok(Rank(3)),
        '4' => Ok(Rank(4)),
        '5' => Ok(Rank(5)),
        '6' => Ok(Rank(6)),
        '7' => Ok(Rank(7)),
        '8' => Ok(Rank(8)),
        '9' => Ok(Rank(9)),
        'T' => Ok(Rank(10)),
        'J' => Ok(Rank(JACK)),
        'Q' => Ok(Rank(QUEEN)),
        'K' => Ok(Rank(KING)),
        // some alternate forms are allowed
        '1' => Ok(Rank(ACE)),
        '0' => Ok(Rank(10)),
        _  => Err(err_could_not_parse_card_value!(value_char))
    }
}

fn parse_suit(suit_char: char) -> Result<Suit, String> {
    match suit_char {
        'C' => Ok(Suit::Club),
        'S' => Ok(Suit::Spade),
        'H' => Ok(Suit::Heart),
        'D' => Ok(Suit::Diamond),
        _  => Err(err_could_not_parse_suit!(suit_char)),
    }
}
