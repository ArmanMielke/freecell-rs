use super::super::rank::rank_from_string;
use super::super::{ACE, JACK, KING, QUEEN};



#[test]
fn test_word_aliases() {
    assert_eq!(Ok(ACE), rank_from_string(String::from("Ace")));
    assert_eq!(Ok(ACE), rank_from_string(String::from("A")));
    assert_eq!(Ok(JACK), rank_from_string(String::from("Jack")));
    assert_eq!(Ok(JACK), rank_from_string(String::from("J")));
    assert_eq!(Ok(QUEEN), rank_from_string(String::from("Queen")));
    assert_eq!(Ok(QUEEN), rank_from_string(String::from("Q")));
    assert_eq!(Ok(KING), rank_from_string(String::from("King")));
    assert_eq!(Ok(KING), rank_from_string(String::from("K")));
}

#[test]
fn test_valid_number() {
    assert_eq!(Ok(ACE), rank_from_string(String::from("1")));
    assert_eq!(Ok(8), rank_from_string(String::from("8")));
    assert_eq!(Ok(KING), rank_from_string(String::from("13")));
}

#[test]
fn test_invalid_number() {
    assert!(rank_from_string(String::from("0")).is_err());
    assert!(rank_from_string(String::from("14")).is_err());
    assert!(rank_from_string(String::from("99")).is_err());
}

#[test]
fn test_not_a_number() {
    assert!(rank_from_string(String::from("not a rank")).is_err());
}
