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
        return Err(format!("Card code \"{}\" is not of length 2.", card_code))
    }

    let mut card_chars = card_code.chars();
    let value_char = card_chars.next().unwrap();
    let suit_char = card_chars.next().unwrap();

    let value = match value_char {
        'A' => Rank(ACE),
        '2' => Rank(2),
        '3' => Rank(3),
        '4' => Rank(4),
        '5' => Rank(5),
        '6' => Rank(6),
        '7' => Rank(7),
        '8' => Rank(8),
        '9' => Rank(9),
        'T' => Rank(10),
        'J' => Rank(JACK),
        'Q' => Rank(QUEEN),
        'K' => Rank(KING),
        // some alternate forms are allowed
        '1' => Rank(ACE),
        '0' => Rank(10),
         _  => return Err(format!("Could not parse card value: {}", value_char))
    };

    let suit = match suit_char {
        'C' => Suit::Club,
        'S' => Suit::Spade,
        'H' => Suit::Heart,
        'D' => Suit::Diamond,
        _  => return Err(format!("Could not parse suit: {}", suit_char)),
    };

    Ok(Card {suit, value,})
}
