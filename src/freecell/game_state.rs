use super::Card;
use super::GameMove;
use super::Cascade;
use super::Foundation;


const NUM_CASCADES: usize = 8;
const NUM_FOUNDATIONS: usize = 4;
const NUM_FREECELLS: usize = 4;


#[derive(Debug, PartialEq)]
pub struct GameState {
    // TODO document
    pub cascades: [Cascade; NUM_CASCADES],

    /// The position of the Foundation in the array determines which suit it holds.
    /// Eg. `foundations[1]` may only hold spade cards, since Suit::Spade equals 1.
    pub foundations: [Foundation; NUM_FOUNDATIONS],

    // TODO replace with vector of fixed length (eg. crate arrayvec)
    // TODO document
    pub freecells: [Option<Card>; NUM_FREECELLS],
}

impl GameState {
    pub fn get_legal_moves(&self) -> Vec<(GameState, GameMove)> {
        panic!("not implemented yet");
    }

    pub fn is_won(&self) -> bool {
        self.foundations[0].len() == 13 &&
        self.foundations[1].len() == 13 &&
        self.foundations[2].len() == 13 &&
        self.foundations[3].len() == 13
    }
}
