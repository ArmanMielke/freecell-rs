use freecell::card::*;


pub fn parse_card(card_string: &str) -> Result<Card, String> {
    if card_string.len() != 2 {
        return Err(format!("Card code \"{}\" is not of length 2.", card_string))
    }

    let mut card_chars = card_string.chars();
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
