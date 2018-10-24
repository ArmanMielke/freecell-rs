use super::card::Card;
use super::card::Suit::{Club, Spade, Heart, Diamond};
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

    // this will be removed later
    pub fn create_empty() -> GameState {
        GameState {
            cascades: [
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
                Cascade(Vec::new()),
            ],
            foundations: [
                Foundation{
                    suit: Club,
                    cards: Vec::new(),
                },
                Foundation{
                    suit: Spade,
                    cards: Vec::new(),
                },
                Foundation{
                    suit: Heart,
                    cards: Vec::new(),
                },
                Foundation{
                    suit: Diamond,
                    cards: Vec::new(),
                },
            ],
            freecells: [
                None, None, None, None,
            ],
        }
    }

    pub fn get_legal_moves(&self) -> String {
        return "not implemented yet".to_string();
    }
}
