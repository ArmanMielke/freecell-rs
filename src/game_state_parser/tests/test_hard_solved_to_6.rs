use arrayvec::ArrayVec;

use crate::freecell::{Card, Foundations, GameState, ACE, JACK, KING, QUEEN};
use crate::freecell::Suit::{Club, Diamond, Heart, Spade};
use super::super::parse_file;



#[test]
fn test_hard_solved_to_6() {
    let expected = GameState {
        cascades: [
            vec![
                Card { suit: Club, rank: QUEEN },
                Card { suit: Spade, rank: QUEEN },
                Card { suit: Club, rank: 7 },
                Card { suit: Heart, rank: 10 },
            ],
            vec![
                Card { suit: Club, rank: 9 },
                Card { suit: Spade, rank: KING },
                Card { suit: Heart, rank: 8 },
                Card { suit: Heart, rank: JACK },
                Card { suit: Club, rank: JACK },
            ],
            vec![
                Card { suit: Heart, rank: 7 },
                Card { suit: Club, rank: KING },
            ],
            vec![
                Card { suit: Diamond, rank: 10 },
                Card { suit: Heart, rank: 9 },
                Card { suit: Spade, rank: 9 },
            ],
            vec![
                Card { suit: Spade, rank: 7 },
                Card { suit: Spade, rank: JACK },
                Card { suit: Diamond, rank: JACK },
            ],
            vec![
                Card { suit: Diamond, rank: 8 },
                Card { suit: Club, rank: 8 },
                Card { suit: Diamond, rank: KING },
                Card { suit: Diamond, rank: QUEEN },
            ],
            vec![
                Card { suit: Spade, rank: 10 },
                Card { suit: Club, rank: 10 },
                Card { suit: Spade, rank: 8 },
            ],
            vec![
                Card { suit: Heart, rank: QUEEN },
                Card { suit: Diamond, rank: 7 },
                Card { suit: Diamond, rank: 9 },
                Card { suit: Heart, rank: KING },
            ],
        ],
        foundations: Foundations([
            vec![
                Card { suit: Club, rank: ACE },
                Card { suit: Club, rank: 2 },
                Card { suit: Club, rank: 3 },
                Card { suit: Club, rank: 4 },
                Card { suit: Club, rank: 5 },
                Card { suit: Club, rank: 6 },
            ],
            vec![
                Card { suit: Spade, rank: ACE },
                Card { suit: Spade, rank: 2 },
                Card { suit: Spade, rank: 3 },
                Card { suit: Spade, rank: 4 },
                Card { suit: Spade, rank: 5 },
                Card { suit: Spade, rank: 6 },
            ],
            vec![
                Card { suit: Heart, rank: ACE },
                Card { suit: Heart, rank: 2 },
                Card { suit: Heart, rank: 3 },
                Card { suit: Heart, rank: 4 },
                Card { suit: Heart, rank: 5 },
                Card { suit: Heart, rank: 6 },
            ],
            vec![
                Card { suit: Diamond, rank: ACE },
                Card { suit: Diamond, rank: 2 },
                Card { suit: Diamond, rank: 3 },
                Card { suit: Diamond, rank: 4 },
                Card { suit: Diamond, rank: 5 },
                Card { suit: Diamond, rank: 6 },
            ],
        ]),
        freecells: ArrayVec::new(),
    };

    let actual = parse_file("test-inputs/hard-solved-to-6.txt").unwrap();

    assert_eq!(actual, expected);
}
