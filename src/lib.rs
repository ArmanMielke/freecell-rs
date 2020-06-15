#![forbid(unsafe_code)]
#![deny(
    anonymous_parameters,
    unused_extern_crates,
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    // TODO [v1] uncomment
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_import_braces,
    unused_qualifications,
)]
// TODO check whether this would be useful
/*
#![warn(
    unreachable_pub,
    missing_doc_code_examples,
)]
*/

// This should generally mirror the contents of README.md, without the badges and the contributing
// and license sections.
//
//! Game objects and rules for the solitaire card game
//! [FreeCell](https://en.wikipedia.org/wiki/FreeCell), written in Rust.
//! The goal of this crate is to aid in the implementation of both a FreeCell game and solvers for
//! FreeCell.
//!
//! # FreeCell Rules
//!
//! **TODO** explain rules (once the explanation in README.md has been improved)
//!
//! # Usage
//!
//! **TODO** copy from README.md (once the description there is finalised)
//!
//! # Serialization
//!
//! If the `"serialization"` feature is enabled, the `Serialize` and `Deserialize` traits from
//! [`serde`](https://docs.rs/serde) are implemented for all types exported by this crate.

mod card;
mod card_collections;
mod game_move;
mod game_state;

pub use self::card::{
    parse_rank,
    Card,
    Color, Colour,
    Rank,
    Suit,
    ACE, JACK, KING, QUEEN
};
pub use self::card_collections::{
    parse_freecells,
    CardCollection,
    Cascade, Cascades,
    Foundation, Foundations,
    Freecell, Freecells
};
pub use self::game_move::{
    Move,
    Position,
    POSITIONS
};
pub use self::game_state::{
    GameState,
    GameStateId
};
