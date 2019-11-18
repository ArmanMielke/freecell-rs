use crate::freecell::Card;
use crate::freecell::Suit::{Club, Spade};
use super::super::parse_file;
use super::super::error_messages::ERR_TOO_MANY_CASCADES;



#[test]
fn test_easy_10_instead_of_t() {
    let actual = parse_file("test-inputs/invalid/easy-10-instead-of-T.txt");
    let expected = Err(err_card_code_not_length_2!("10S"));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_duplicate_card() {
    let actual = parse_file("test-inputs/invalid/easy-duplicate-card.txt");
    let expected = Err(err_invgs_card_does_not_exist_exactly_once!(
        Card { suit: Club, value: 10 },
        2
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_extra_cascade() {
    let actual = parse_file("test-inputs/invalid/easy-extra-cascade.txt");
    let expected = Err(String::from(ERR_TOO_MANY_CASCADES));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_card_value() {
    let actual = parse_file("test-inputs/invalid/easy-invalid-card-value.txt");
    let expected = Err(err_could_not_parse_card_value!('X'));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_suit() {
    let actual = parse_file("test-inputs/invalid/easy-invalid-suit.txt");
    let expected = Err(err_could_not_parse_suit!('X'));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_missing_card() {
    let actual = parse_file("test-inputs/invalid/easy-missing-card.txt");
    let expected = Err(err_invgs_card_does_not_exist_exactly_once!(
        Card { suit: Spade, value: 10 },
        0
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_misspelled_cascade() {
    let actual = parse_file("test-inputs/invalid/easy-misspelled-cascade.txt");
    let expected = Err(err_invgs_card_does_not_exist_exactly_once!(
        Card { suit: Club, value: 4 },
        0
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_duplicate_foundation() {
    // TODO (test file does not exist yet)
}
