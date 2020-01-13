use freecell::Suit::{Club, Diamond, Spade};
use freecell::{Card, GameState};

#[test]
fn test_easy_duplicate_card() {
    let actual = GameState::from_file("test-inputs/invalid/easy-duplicate-card.txt");
    let expected = Err(format!(
        "Card {} exists {} times, should exist once",
        Card { suit: Club, rank: 10 }, 2
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_extra_cascade() {
    let actual = GameState::from_file("test-inputs/invalid/easy-extra-cascade.txt");
    let expected = Err(String::from("Too many cascades"));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_card_rank() {
    let actual = GameState::from_file("test-inputs/invalid/easy-invalid-card-rank.txt");
    let expected = Err("Could not parse cascade: \" 3S 8S AD 2D AS 4C XS\"".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_suit() {
    let actual = GameState::from_file("test-inputs/invalid/easy-invalid-suit.txt");
    let expected = Err("Could not parse cascade: \" 3S 8S AD 2D AS 4C TX\"".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_missing_card() {
    let actual = GameState::from_file("test-inputs/invalid/easy-missing-card.txt");
    let expected = Err(format!(
        "Card {} exists {} times, should exist once",
        Card { suit: Spade, rank: 10 }, 0
    ));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_misspelled_cascade() {
    let actual = GameState::from_file("test-inputs/invalid/easy-misspelled-cascade.txt");
    let expected = Err("Line starts with invalid token: \"cscade\"".to_string());
    assert_eq!(actual, expected);
}

// TODO [v1] rethink what the expected behaviour is in this case
//#[test]
fn test_hard_solved_to_2_too_many_foundations_in_multiple_lines() {
    let actual = GameState::from_file("test-inputs/invalid/hard-solved-to-2-too-many-foundations-in-multiple-lines.txt");
    let expected = Err(format!("Multiple foundations of suit {} specified", Diamond));
    assert_eq!(actual, expected);
}

#[test]
fn test_hard_solved_to_2_too_many_foundations_in_one_line() {
    let actual = GameState::from_file("test-inputs/invalid/hard-solved-to-2-too-many-foundations-in-one-line.txt");
    let expected = Err("Could not parse foundations: \" 2C 2S 2H 2D 3C\"".to_string());
    assert_eq!(actual, expected);
}

#[test]
fn test_hard_solved_to_2_multiple_foundations_of_the_same_suit() {
    let actual = GameState::from_file("test-inputs/invalid/hard-solved-to-2-multiple-foundations-of-the-same-suit.txt");
    let expected = Err(format!("Multiple foundations of suit {} specified", Club));
    assert_eq!(actual, expected);
}
