use std::convert::TryFrom;

use super::super::Suit;
use super::super::Suit::{Club, Diamond, Heart, Spade};



#[test]
fn test_valid_name() {
    assert_eq!(Ok(Club), Suit::try_from(String::from("Club")));
    assert_eq!(Ok(Club), Suit::try_from(String::from("Clubs")));
    assert_eq!(Ok(Club), Suit::try_from(String::from("C")));
    assert_eq!(Ok(Spade), Suit::try_from(String::from("Spade")));
    assert_eq!(Ok(Spade), Suit::try_from(String::from("Spades")));
    assert_eq!(Ok(Spade), Suit::try_from(String::from("S")));
    assert_eq!(Ok(Heart), Suit::try_from(String::from("Heart")));
    assert_eq!(Ok(Heart), Suit::try_from(String::from("Hearts")));
    assert_eq!(Ok(Heart), Suit::try_from(String::from("H")));
    assert_eq!(Ok(Diamond), Suit::try_from(String::from("Diamond")));
    assert_eq!(Ok(Diamond), Suit::try_from(String::from("Diamonds")));
    assert_eq!(Ok(Diamond), Suit::try_from(String::from("D")));
}

#[test]
fn test_invalid_name() {
    assert!(Suit::try_from(String::from("Not A Suit")).is_err());
    assert!(Suit::try_from(String::from("He")).is_err());
    assert!(Suit::try_from(String::from("Hear")).is_err());
    assert!(Suit::try_from(String::from("Heartss")).is_err());
}
