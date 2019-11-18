use arrayvec::ArrayVec;

use crate::freecell::GameState;
use crate::freecell::card::{Card, ACE, JACK, KING, QUEEN};
use crate::freecell::card::Suit::{Club, Diamond, Heart, Spade};
use super::super::parse_file;



#[test]
fn test_solved() {
    let expected = GameState {
        cascades: [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ],
        foundations: [
            vec![
                Card { suit: Club, value: ACE },
                Card { suit: Club, value: 2 },
                Card { suit: Club, value: 3 },
                Card { suit: Club, value: 4 },
                Card { suit: Club, value: 5 },
                Card { suit: Club, value: 6 },
                Card { suit: Club, value: 7 },
                Card { suit: Club, value: 8 },
                Card { suit: Club, value: 9 },
                Card { suit: Club, value: 10 },
                Card { suit: Club, value: JACK },
                Card { suit: Club, value: QUEEN },
                Card { suit: Club, value: KING },
            ],
            vec![
                Card { suit: Spade, value: ACE },
                Card { suit: Spade, value: 2 },
                Card { suit: Spade, value: 3 },
                Card { suit: Spade, value: 4 },
                Card { suit: Spade, value: 5 },
                Card { suit: Spade, value: 6 },
                Card { suit: Spade, value: 7 },
                Card { suit: Spade, value: 8 },
                Card { suit: Spade, value: 9 },
                Card { suit: Spade, value: 10 },
                Card { suit: Spade, value: JACK },
                Card { suit: Spade, value: QUEEN },
                Card { suit: Spade, value: KING },
            ],
            vec![
                Card { suit: Heart, value: ACE },
                Card { suit: Heart, value: 2 },
                Card { suit: Heart, value: 3 },
                Card { suit: Heart, value: 4 },
                Card { suit: Heart, value: 5 },
                Card { suit: Heart, value: 6 },
                Card { suit: Heart, value: 7 },
                Card { suit: Heart, value: 8 },
                Card { suit: Heart, value: 9 },
                Card { suit: Heart, value: 10 },
                Card { suit: Heart, value: JACK },
                Card { suit: Heart, value: QUEEN },
                Card { suit: Heart, value: KING },
            ],
            vec![
                Card { suit: Diamond, value: ACE },
                Card { suit: Diamond, value: 2 },
                Card { suit: Diamond, value: 3 },
                Card { suit: Diamond, value: 4 },
                Card { suit: Diamond, value: 5 },
                Card { suit: Diamond, value: 6 },
                Card { suit: Diamond, value: 7 },
                Card { suit: Diamond, value: 8 },
                Card { suit: Diamond, value: 9 },
                Card { suit: Diamond, value: 10 },
                Card { suit: Diamond, value: JACK },
                Card { suit: Diamond, value: QUEEN },
                Card { suit: Diamond, value: KING },
            ],
        ],
        freecells: ArrayVec::new(),
    };

    let actual = parse_file("test-inputs/solved.txt").unwrap();

    assert_eq!(actual, expected);
}
