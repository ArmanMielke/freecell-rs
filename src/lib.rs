#![forbid(unsafe_code)]

//! **TODO** general introduction
//!
//! # FreeCell Rules
//!
//! **TODO** explain rules (once the explanation in README.md has been improved)
//!
//! # Serialization
//!
//! If the `"serialization"` feature is enabled, the `Serialize` and `Deserialize` traits from
//! [`serde`](https://docs.rs/serde) are implemented for all types exported by this crate.

pub mod game_state_parser;

mod card;
mod card_collections;
mod game_move;
mod game_state;

pub use self::card::{rank_from_string, Card, Color, Colour, Rank, Suit, ACE, JACK, KING, QUEEN};
pub use self::card_collections::{CardCollection, Cascade, Cascades, Foundation, Foundations, Freecells};
pub use self::game_move::{Move, Position};
pub use self::game_state::{GameState, GameStateId};
