#![forbid(unsafe_code)]

pub mod game_state_parser;

mod card;
mod card_collection;
mod cascade;
mod foundation;
mod freecells;
mod game_move;
mod game_state;
mod position;

pub use self::card::{rank_from_string, Card, Color, Colour, Rank, Suit, ACE, JACK, KING, QUEEN};
pub use self::card_collection::CardCollection;
pub use self::cascade::{Cascade, Cascades};
pub use self::foundation::{Foundation, Foundations};
pub use self::freecells::Freecells;
pub use self::game_move::Move;
pub use self::game_state::{GameState, GameStateId};
pub use self::position::Position;
