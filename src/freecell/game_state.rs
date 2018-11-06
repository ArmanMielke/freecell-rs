use super::card::Card;
use super::cascade::Cascade;
use super::foundation::Foundation;


const NUM_CASCADES: usize = 8;
const NUM_FOUNDATIONS: usize = 4;
const NUM_FREECELLS: usize = 4;


pub struct GameState {
    pub cascades: [Cascade; NUM_CASCADES],
    pub foundations: [Foundation; NUM_FOUNDATIONS],
    pub freecells: [Option<Card>; NUM_FREECELLS],
}

impl GameState {
    pub fn get_legal_moves(&self) -> String {
        return "not implemented yet".to_string();
    }

    pub fn is_won(&self) -> bool {
        self.foundations[0].cards.len() == 13 &&
        self.foundations[1].cards.len() == 13 &&
        self.foundations[2].cards.len() == 13 &&
        self.foundations[3].cards.len() == 13
    }
}