use super::{Card, GameMove, Cascades, Foundations};

use arrayvec::ArrayVec;



/// May hold up to four arbitrary cards
pub type Freecells = ArrayVec<[Card; 4]>;


#[derive(Debug, PartialEq)]
pub struct GameState {
    pub cascades: Cascades,
    pub foundations: Foundations,
    pub freecells: Freecells,
}

impl GameState {
    pub fn get_legal_moves(&self) -> Vec<(GameState, GameMove)> {
        panic!("not implemented yet");
    }

    pub fn is_solved(&self) -> bool {
        self.foundations[0].len() == 13 &&
        self.foundations[1].len() == 13 &&
        self.foundations[2].len() == 13 &&
        self.foundations[3].len() == 13
    }
}
