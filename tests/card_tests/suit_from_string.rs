use freecell::Suit::{self, Club, Diamond, Heart, Spade};

#[test]
fn test_valid_name() {
    assert_eq!(Ok(Club), "Club".parse());
    assert_eq!(Ok(Club), "Clubs".parse());
    assert_eq!(Ok(Club), "c".parse());
    assert_eq!(Ok(Spade), "Spade".parse());
    assert_eq!(Ok(Spade), "SPADES".parse());
    assert_eq!(Ok(Spade), "S".parse());
    assert_eq!(Ok(Heart), "Heart".parse());
    assert_eq!(Ok(Heart), "hearts".parse());
    assert_eq!(Ok(Heart), "H".parse());
    assert_eq!(Ok(Diamond), "Diamond".parse());
    assert_eq!(Ok(Diamond), "Diamonds".parse());
    assert_eq!(Ok(Diamond), "d".parse());
}

#[test]
fn test_invalid_name() {
    assert!("Not A Suit".parse::<Suit>().is_err());
    assert!("He".parse::<Suit>().is_err());
    assert!("Hear".parse::<Suit>().is_err());
    assert!("Heartss".parse::<Suit>().is_err());
}
