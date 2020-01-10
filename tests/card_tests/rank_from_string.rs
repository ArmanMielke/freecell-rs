use freecell::{parse_rank, ACE, JACK, KING, QUEEN};

#[test]
fn test_word_aliases() {
    assert_eq!(Ok(ACE), parse_rank("Ace"));
    assert_eq!(Ok(ACE), parse_rank("A"));
    assert_eq!(Ok(10), parse_rank("T"));
    assert_eq!(Ok(JACK), parse_rank("Jack"));
    assert_eq!(Ok(JACK), parse_rank("j"));
    assert_eq!(Ok(QUEEN), parse_rank("Queen"));
    assert_eq!(Ok(QUEEN), parse_rank("Q"));
    assert_eq!(Ok(KING), parse_rank("King"));
    assert_eq!(Ok(KING), parse_rank("k"));
}

#[test]
fn test_valid_number() {
    assert_eq!(Ok(ACE), parse_rank("1"));
    assert_eq!(Ok(8), parse_rank("8"));
    assert_eq!(Ok(10), parse_rank("10"));
    assert_eq!(Ok(KING), parse_rank("13"));
}

#[test]
fn test_invalid_number() {
    assert!(parse_rank("0").is_err());
    assert!(parse_rank("14").is_err());
    assert!(parse_rank("99").is_err());
}

#[test]
fn test_not_a_number() {
    assert!(parse_rank("not a rank").is_err());
}
