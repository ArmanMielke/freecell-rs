use super::super::rank::rank_from_string;
use super::super::{ACE, JACK, KING, QUEEN};



#[test]
fn test_word_aliases() {
    assert_eq!(ACE, rank_from_string(String::from("Ace")).unwrap());
    assert_eq!(ACE, rank_from_string(String::from("A")).unwrap());
    assert_eq!(JACK, rank_from_string(String::from("Jack")).unwrap());
    assert_eq!(JACK, rank_from_string(String::from("J")).unwrap());
    assert_eq!(QUEEN, rank_from_string(String::from("Queen")).unwrap());
    assert_eq!(QUEEN, rank_from_string(String::from("Q")).unwrap());
    assert_eq!(KING, rank_from_string(String::from("King")).unwrap());
    assert_eq!(KING, rank_from_string(String::from("K")).unwrap());
}

#[test]
fn test_valid_number() {
    assert_eq!(ACE, rank_from_string(String::from("1")).unwrap());
    assert_eq!(8, rank_from_string(String::from("8")).unwrap());
    assert_eq!(KING, rank_from_string(String::from("13")).unwrap());
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
