use freecell::game_state_parser::parse_file;
use freecell::Suit::{Club, Diamond, Spade};
use freecell::{Card};

#[test]
fn test_easy_10_instead_of_t() {
    // TODO [low priority] test for exact error message
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
    let expected = Err(String::from("Too many cascades"));
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
fn test_hard_solved_to_2_too_many_foundations_in_multiple_lines() {
    let actual = parse_file("test-inputs/invalid/hard-solved-to-2-too-many-foundations-in-multiple-lines.txt");
    let expected = Err(format!("Multiple foundations of suit {} specified", Diamond));
    assert_eq!(actual, expected);
}

#[test]
fn test_hard_solved_to_2_too_many_foundations_in_one_line() {
    let actual = parse_file("test-inputs/invalid/hard-solved-to-2-too-many-foundations-in-one-line.txt");
    let expected = Err(format!("Multiple foundations of suit {} specified", Club));
    assert_eq!(actual, expected);
}
