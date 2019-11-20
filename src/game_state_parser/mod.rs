//!
//! Contains the function parse_file(...) which reads a text file containing a game state and
//! converts it into a GameState object.
//!

mod conversions_to_array;
mod error_messages;
mod parse_file;

pub use self::parse_file::parse_file;

#[cfg(test)]
mod tests;
