use std::convert::TryFrom;

use super::super::Suit;
use super::super::Suit::{Club, Diamond, Heart, Spade};



#[test]
fn test_correct_name() {
    assert_eq!(Ok(Club), Suit::try_from(String::from("Club")));
    assert_eq!(Ok(Club), Suit::try_from(String::from("C")));
    assert_eq!(Ok(Spade), Suit::try_from(String::from("Spade")));
    assert_eq!(Ok(Spade), Suit::try_from(String::from("S")));
    assert_eq!(Ok(Heart), Suit::try_from(String::from("Heart")));
    assert_eq!(Ok(Heart), Suit::try_from(String::from("H")));
    assert_eq!(Ok(Diamond), Suit::try_from(String::from("Diamond")));
    assert_eq!(Ok(Diamond), Suit::try_from(String::from("D")));
}

#[test]
fn test_incorrect_name() {
    assert!(Suit::try_from(String::from("Not A Suit")).is_err());
}
