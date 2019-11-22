use super::super::rank::rank_from_string;
use super::super::{ACE, JACK, KING, QUEEN};

#[test]
fn test_word_aliases() {
    assert_eq!(Ok(ACE), rank_from_string("Ace"));
    assert_eq!(Ok(ACE), rank_from_string("A"));
    assert_eq!(Ok(10), rank_from_string("T"));
    assert_eq!(Ok(JACK), rank_from_string("Jack"));
    assert_eq!(Ok(JACK), rank_from_string("j"));
    assert_eq!(Ok(QUEEN), rank_from_string("Queen"));
    assert_eq!(Ok(QUEEN), rank_from_string("Q"));
    assert_eq!(Ok(KING), rank_from_string("King"));
    assert_eq!(Ok(KING), rank_from_string("k"));
}

#[test]
fn test_valid_number() {
    assert_eq!(Ok(ACE), rank_from_string("1"));
    assert_eq!(Ok(8), rank_from_string("8"));
    assert_eq!(Ok(10), rank_from_string("10"));
    assert_eq!(Ok(KING), rank_from_string("13"));
}

#[test]
fn test_invalid_number() {
    assert!(rank_from_string("0").is_err());
    assert!(rank_from_string("14").is_err());
    assert!(rank_from_string("99").is_err());
}

#[test]
fn test_not_a_number() {
    assert!(rank_from_string("not a rank").is_err());
}
