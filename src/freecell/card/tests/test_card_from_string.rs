use std::convert::TryFrom;

use super::super::{Card, ACE, JACK, KING, QUEEN};
use super::super::Suit::{Club, Diamond, Heart, Spade};



#[test]
fn test_long_representation() {
    // need to test all named ranks, since they are not tested elsewhere
    assert_eq!(
        Ok(Card { suit: Spade, rank: ACE }),
        Card::try_from(String::from("Ace of Spades"))
    );
    assert_eq!(
        Ok(Card { suit: Diamond, rank: JACK }),
        Card::try_from(String::from("Jack of Diamonds"))
    );
    assert_eq!(
        Ok(Card { suit: Heart, rank: QUEEN }),
        Card::try_from(String::from("Queen of Hearts"))
    );
    assert_eq!(
        Ok(Card { suit: Club, rank: KING }),
        Card::try_from(String::from("King of Clubs"))
    );

    // numbers as ranks
    assert_eq!(
        Ok(Card { suit: Heart, rank: 8 }),
        Card::try_from(String::from("8 of Hearts"))
    );
    assert_eq!(
        Ok(Card { suit: Club, rank: 10 }),
        Card::try_from(String::from("10 of Clubs"))
    );
    assert_eq!(
        Ok(Card { suit: Spade, rank: JACK }),
        Card::try_from(String::from("11 of Spades"))
    );
}

#[test]
fn test_short_representation() {
    // need to test all named ranks, since they are not tested elsewhere
    assert_eq!(
        Ok(Card { suit: Diamond, rank: ACE }),
        Card::try_from(String::from("AD"))
    );
    // 10 is a named rank in the short representation
    assert_eq!(
        Ok(Card { suit: Diamond, rank: 10 }),
        Card::try_from(String::from("TD"))
    );
    assert_eq!(
        Ok(Card { suit: Club, rank: JACK }),
        Card::try_from(String::from("JC"))
    );
    assert_eq!(
        Ok(Card { suit: Spade, rank: QUEEN }),
        Card::try_from(String::from("QS"))
    );
    assert_eq!(
        Ok(Card { suit: Heart, rank: KING }),
        Card::try_from(String::from("KH"))
    );

    // number as rank
    assert_eq!(
        Ok(Card { suit: Club, rank: 4 }),
        Card::try_from(String::from("4C"))
    );
}

#[test]
fn test_invalid() {
    assert!(Card::try_from(String::from("Not A Card")).is_err());
    assert!(Card::try_from(String::from("Jack of NotASuit")).is_err());
    assert!(Card::try_from(String::from("NotARank of Spades")).is_err());
    assert!(Card::try_from(String::from("0S")).is_err()); // invalid rank
    assert!(Card::try_from(String::from("XC")).is_err()); // invalid rank
    assert!(Card::try_from(String::from("6X")).is_err()); // invalid suit
}
