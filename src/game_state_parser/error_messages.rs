#![macro_use]

// regular error messages
pub const ERR_COULD_NOT_READ_FILE: &str = "File could not be read";
pub const ERR_COULD_NOT_READ_FILE_CONTENTS: &str = "File contents could not be read";
pub const ERR_TOO_MANY_CASCADES: &str = "Too many cascades";
pub const ERR_TOO_MANY_FREECELLS: &str = "Too many freecells";


// error messages that use formatting
macro_rules! err_multiple_foundations_of_suit {
    ($suit: expr) => (format!("Multiple foundations of suit {} specified", $suit))
}


// invgs stands for invalid game state
macro_rules! err_invgs_card_does_not_exist_exactly_once {
    ($card: expr, $count: expr) => (
        format!("Card {} exists {} times, should exist once", $card, $count)
    )
}

macro_rules! err_invgs_foundation_wrong_suit {
    ($foundation_suit: expr, $card_suit: expr, $card: expr) => (
        format!(
            "Foundation of suit {} contains card of suit {}: {}",
            $foundation_suit, $card_suit, $card
        )
    )
}

macro_rules! err_invgs_foundation_wrong_order {
    ($foundation_suit: expr, $card_rank: expr, $position: expr) => (
        format!(
            "Foundation of suit {} contains card of rank {} in position {}",
            $foundation_suit, $card_rank, $position
        )
    )
}


// error messages that print themselves
macro_rules! warn_invalid_first_token {
    ($token: expr) => (eprintln!("Line starts with invalid token: {}", $token))
}
