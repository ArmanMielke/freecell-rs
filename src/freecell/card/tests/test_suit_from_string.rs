use std::convert::TryFrom;

use super::super::Suit;
use super::super::Suit::{Club, Diamond, Heart, Spade};



#[test]
fn test_valid_name() {
    assert_eq!(Ok(Club), Suit::try_from("Club"));
    assert_eq!(Ok(Club), Suit::try_from("Clubs"));
    assert_eq!(Ok(Club), Suit::try_from("c"));
    assert_eq!(Ok(Spade), Suit::try_from("Spade"));
    assert_eq!(Ok(Spade), Suit::try_from("SPADES"));
    assert_eq!(Ok(Spade), Suit::try_from("S"));
    assert_eq!(Ok(Heart), Suit::try_from("Heart"));
    assert_eq!(Ok(Heart), Suit::try_from("hearts"));
    assert_eq!(Ok(Heart), Suit::try_from("H"));
    assert_eq!(Ok(Diamond), Suit::try_from("Diamond"));
    assert_eq!(Ok(Diamond), Suit::try_from("Diamonds"));
    assert_eq!(Ok(Diamond), Suit::try_from("d"));
}

#[test]
fn test_invalid_name() {
    assert!(Suit::try_from("Not A Suit").is_err());
    assert!(Suit::try_from("He").is_err());
    assert!(Suit::try_from("Hear").is_err());
    assert!(Suit::try_from("Heartss").is_err());
}
