#![forbid(unsafe_code)]
#![deny(
    anonymous_parameters,
    unused_extern_crates,
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    // TODO uncomment
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_import_braces,
    unused_qualifications,
)]
// TODO check whether this would be useful
// #![warn(unreachable_pub)]

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
pub use self::card_collections::{CardCollection, Cascade, Cascades, Foundation, Foundations, Freecell, Freecells};
pub use self::game_move::{Move, Position, POSITIONS};
pub use self::game_state::{GameState, GameStateId};
