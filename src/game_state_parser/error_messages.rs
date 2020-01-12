#![macro_use]

// regular error messages
pub const ERR_COULD_NOT_READ_FILE: &str = "File could not be read";
pub const ERR_COULD_NOT_READ_FILE_CONTENTS: &str = "File contents could not be read";
pub const ERR_TOO_MANY_CASCADES: &str = "Too many cascades";

macro_rules! warn_invalid_first_token {
    ($token: expr) => {
        eprintln!("Line starts with invalid token: {}", $token)
    };
}
