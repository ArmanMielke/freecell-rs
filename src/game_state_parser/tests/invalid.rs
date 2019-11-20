use crate::freecell::Card;
use crate::freecell::Suit::{Club, Spade};
use super::super::parse_file;
use super::super::error_messages::ERR_TOO_MANY_CASCADES;



#[test]
fn test_easy_10_instead_of_t() {
    // TODO test for exact error message
    assert!(parse_file("test-inputs/invalid/easy-10-instead-of-T.txt").is_err());
}

#[test]
fn test_easy_duplicate_card() {
    let actual = parse_file("test-inputs/invalid/easy-duplicate-card.txt");
    let expected = Err(format!(
        "Card {} exists {} times, should exist once",
        Card { suit: Club, rank: 10 }, 2
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
fn test_easy_invalid_card_rank() {
    let actual = parse_file("test-inputs/invalid/easy-invalid-card-rank.txt");
    let expected = Err("Rank is neither named rank nor integer: \"X\"".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_suit() {
    let actual = parse_file("test-inputs/invalid/easy-invalid-suit.txt");
    let expected = Err("Suit \"X\" does not match any of \"C[lub[s]]\", \"S[pade[s]]\", \"H[eart[s]]\", \"D[iamond[s]]\" (case-insensitive)".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_missing_card() {
    let actual = parse_file("test-inputs/invalid/easy-missing-card.txt");
    let expected = Err(format!(
        "Card {} exists {} times, should exist once",
        Card { suit: Spade, rank: 10 }, 0
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_misspelled_cascade() {
    let actual = parse_file("test-inputs/invalid/easy-misspelled-cascade.txt");
    let expected = Err(format!(
        "Card {} exists {} times, should exist once",
        Card { suit: Club, rank: 4 }, 0
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_duplicate_foundation() {
    // TODO (test file does not exist yet)
}
