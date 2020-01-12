use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, ACE, JACK, KING, QUEEN};

#[test]
fn test_long_representation() {
    // named ranks
    assert_eq!(Ok(Card { suit: Spade, rank: ACE }), "Ace of Spades".parse());
    assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), "Jack of Diamonds".parse());
    assert_eq!(Ok(Card { suit: Heart, rank: QUEEN }), "Queen of Hearts".parse());
    assert_eq!(Ok(Card { suit: Club, rank: KING }), "King of Clubs".parse());

    // numbers as ranks
    assert_eq!(Ok(Card { suit: Heart, rank: 8 }), "8 of Hearts".parse());
    assert_eq!(Ok(Card { suit: Club, rank: 10 }), "10 of Clubs".parse());
    assert_eq!(Ok(Card { suit: Spade, rank: JACK }), "11 of Spades".parse());
}

#[test]
fn test_short_representation() {
    // named ranks
    assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), "ad".parse());
    // 10 is a named rank in the short representation
    assert_eq!(Ok(Card { suit: Diamond, rank: 10 }), "TD".parse());
    assert_eq!(Ok(Card { suit: Club, rank: JACK }), "JC".parse());
    assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), "qS".parse());
    assert_eq!(Ok(Card { suit: Heart, rank: KING }), "Kh".parse());

    // number as rank
    assert_eq!(Ok(Card { suit: Club, rank: 4 }), "4C".parse());
}

#[test]
fn test_invalid() {
    assert!("Not A Card".parse::<Card>().is_err());
    assert!("Jack of NotASuit".parse::<Card>().is_err());
    assert!("NotARank of Spades".parse::<Card>().is_err());
    assert!("0S".parse::<Card>().is_err()); // invalid rank
    assert!("XC".parse::<Card>().is_err()); // invalid rank
    assert!("6X".parse::<Card>().is_err()); // invalid suit
}
