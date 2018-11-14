#![macro_use]

// regular error messages
pub const ERR_COULD_NOT_READ_FILE: &str = "File could not be read";
pub const ERR_COULD_NOT_READ_FILE_CONTENTS: &str = "File contents could not be read";
pub const ERR_TOO_MANY_CASCADES: &str = "Too many cascades";
pub const ERR_TOO_MANY_FREECELLS: &str = "Too many freecells";

// error messages that use formatting
macro_rules! err_card_code_not_length_2 {
    ($card_code: expr) => (format!("Card code \"{}\" is not of length 2", $card_code))
}

macro_rules! err_could_not_parse_card_value {
    ($value: expr) => (format!("Could not parse card value: {}", $value))
}

macro_rules! err_could_not_parse_suit {
    ($suit: expr) => (format!("Could not parse suit: {}", $suit))
}

macro_rules! err_multiple_foundations_of_suit {
    ($suit: expr) => (format!("Multiple foundations of suit {:?} specified", $suit))
}

macro_rules! warn_invalid_first_token {
    ($token: expr) => (eprintln!("Line starts with invalid token: {}", $token))
}
