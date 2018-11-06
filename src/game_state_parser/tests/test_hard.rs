use freecell::{Foundation, GameState};
use freecell::card::*;
use super::super::parse_file;


#[test]
fn test_hard() {
    let expected = GameState {
        cascades: [
            vec![
                Card { suit: Suit::Club, value: 6 },
                Card { suit: Suit::Club, value: QUEEN },
                Card { suit: Suit::Diamond, value: 5 },
                Card { suit: Suit::Spade, value: QUEEN },
                Card { suit: Suit::Club, value: 7 },
                Card { suit: Suit::Heart, value: 10 },
                Card { suit: Suit::Spade, value: 4 },
            ],
            vec![
                Card { suit: Suit::Club, value: 9 },
                Card { suit: Suit::Heart, value: 3 },
                Card { suit: Suit::Spade, value: KING },
                Card { suit: Suit::Heart, value: 8 },
                Card { suit: Suit::Heart, value: JACK },
                Card { suit: Suit::Spade, value: 2 },
                Card { suit: Suit::Club, value: JACK },
            ],
            vec![
                Card { suit: Suit::Club, value: 5 },
                Card { suit: Suit::Heart, value: 7 },
                Card { suit: Suit::Heart, value: 6 },
                Card { suit: Suit::Diamond, value: ACE },
                Card { suit: Suit::Spade, value: ACE },
                Card { suit: Suit::Heart, value: ACE },
                Card { suit: Suit::Club, value: KING },
            ],
            vec![
                Card { suit: Suit::Diamond, value: 10 },
                Card { suit: Suit::Heart, value: 9 },
                Card { suit: Suit::Heart, value: 5 },
                Card { suit: Suit::Spade, value: 9 },
                Card { suit: Suit::Diamond, value: 6 },
                Card { suit: Suit::Spade, value: 6 },
                Card { suit: Suit::Club, value: 3 },
            ],
            vec![
                Card { suit: Suit::Spade, value: 7 },
                Card { suit: Suit::Spade, value: JACK },
                Card { suit: Suit::Diamond, value: JACK },
                Card { suit: Suit::Club, value: ACE },
                Card { suit: Suit::Diamond, value: 2 },
                Card { suit: Suit::Spade, value: 3 },
            ],
            vec![
                Card { suit: Suit::Diamond, value: 3 },
                Card { suit: Suit::Club, value: 2 },
                Card { suit: Suit::Diamond, value: 8 },
                Card { suit: Suit::Club, value: 8 },
                Card { suit: Suit::Diamond, value: KING },
                Card { suit: Suit::Diamond, value: QUEEN },
            ],
            vec![
                Card { suit: Suit::Spade, value: 10 },
                Card { suit: Suit::Heart, value: 4 },
                Card { suit: Suit::Heart, value: 2 },
                Card { suit: Suit::Club, value: 10 },
                Card { suit: Suit::Club, value: 4 },
                Card { suit: Suit::Spade, value: 8 },
            ],
            vec![
                Card { suit: Suit::Heart, value: QUEEN },
                Card { suit: Suit::Diamond, value: 4 },
                Card { suit: Suit::Spade, value: 5 },
                Card { suit: Suit::Diamond, value: 7 },
                Card { suit: Suit::Diamond, value: 9 },
                Card { suit: Suit::Heart, value: KING },
            ],
        ],
        foundations: [
            Foundation {
                cards: Vec::new(),
                suit: Suit::Club,
            },
            Foundation {
                cards: Vec::new(),
                suit: Suit::Spade,
            },
            Foundation {
                cards: Vec::new(),
                suit: Suit::Heart,
            },
            Foundation {
                cards: Vec::new(),
                suit: Suit::Diamond,
            },
        ],
        freecells: [None, None, None, None],
    };

    let actual = parse_file("example-inputs/hard.txt").unwrap();

    assert_eq!(actual, expected);
}
