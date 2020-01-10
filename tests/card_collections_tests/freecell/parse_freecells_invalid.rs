use freecell::parse_freecells;

#[test]
fn test_invalid_card() {
    assert_eq!(
        parse_freecells("KD XX 8H"),
        Err("Could not parse freecells".to_string())
    );
}

#[test]
fn test_comma_separated() {
    assert_eq!(
        parse_freecells("JH, TD, 9H"),
        Err("Could not parse freecells".to_string())
    );
}

#[test]
fn test_too_many_cards() {
    assert_eq!(
        parse_freecells("JH TD 9H 6C 5S"),
        Err("Could not parse freecells".to_string())
    );
}
