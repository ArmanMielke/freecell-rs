use std::convert::TryFrom;

use freecell::Suit::{self, Club, Diamond, Heart, Spade};

#[test]
fn test_display() {
    assert_eq!(Ok(Club), Suit::try_from(Club.to_string()));
    assert_eq!(Ok(Spade), Suit::try_from(Spade.to_string()));
    assert_eq!(Ok(Heart), Suit::try_from(Heart.to_string()));
    assert_eq!(Ok(Diamond), Suit::try_from(Diamond.to_string()));
}

#[test]
fn test_debug() {
    assert_eq!(Ok(Club), Suit::try_from(format!("{:?}", Club)));
    assert_eq!(Ok(Spade), Suit::try_from(format!("{:?}", Spade)));
    assert_eq!(Ok(Heart), Suit::try_from(format!("{:?}", Heart)));
    assert_eq!(Ok(Diamond), Suit::try_from(format!("{:?}", Diamond)));
}
