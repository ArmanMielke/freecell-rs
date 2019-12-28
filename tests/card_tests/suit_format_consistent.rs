use freecell::Suit::{Club, Diamond, Heart, Spade};

#[test]
fn test_display() {
    assert_eq!(Ok(Club), Club.to_string().parse());
    assert_eq!(Ok(Spade), Spade.to_string().parse());
    assert_eq!(Ok(Heart), Heart.to_string().parse());
    assert_eq!(Ok(Diamond), Diamond.to_string().parse());
}

#[test]
fn test_debug() {
    assert_eq!(Ok(Club), format!("{:?}", Club).parse());
    assert_eq!(Ok(Spade), format!("{:?}", Spade).parse());
    assert_eq!(Ok(Heart), format!("{:?}", Heart).parse());
    assert_eq!(Ok(Diamond), format!("{:?}", Diamond).parse());
}
