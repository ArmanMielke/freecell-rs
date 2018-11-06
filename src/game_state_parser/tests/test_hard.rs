use freecell::{Cascade, Foundation, GameState};
use freecell::card::*;
use super::super::parse_file;


#[test]
fn test_hard() {
    let expected = GameState {
        cascades: [
            Cascade(vec![
                Card { suit: Suit::Club, value: Rank(6) },
                Card { suit: Suit::Club, value: Rank(QUEEN) },
                Card { suit: Suit::Diamond, value: Rank(5) },
                Card { suit: Suit::Spade, value: Rank(QUEEN) },
                Card { suit: Suit::Club, value: Rank(7) },
                Card { suit: Suit::Heart, value: Rank(10) },
                Card { suit: Suit::Spade, value: Rank(4) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Club, value: Rank(9) },
                Card { suit: Suit::Heart, value: Rank(3) },
                Card { suit: Suit::Spade, value: Rank(KING) },
                Card { suit: Suit::Heart, value: Rank(8) },
                Card { suit: Suit::Heart, value: Rank(JACK) },
                Card { suit: Suit::Spade, value: Rank(2) },
                Card { suit: Suit::Club, value: Rank(JACK) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Club, value: Rank(5) },
                Card { suit: Suit::Heart, value: Rank(7) },
                Card { suit: Suit::Heart, value: Rank(6) },
                Card { suit: Suit::Diamond, value: Rank(ACE) },
                Card { suit: Suit::Spade, value: Rank(ACE) },
                Card { suit: Suit::Heart, value: Rank(ACE) },
                Card { suit: Suit::Club, value: Rank(KING) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Diamond, value: Rank(10) },
                Card { suit: Suit::Heart, value: Rank(9) },
                Card { suit: Suit::Heart, value: Rank(5) },
                Card { suit: Suit::Spade, value: Rank(9) },
                Card { suit: Suit::Diamond, value: Rank(6) },
                Card { suit: Suit::Spade, value: Rank(6) },
                Card { suit: Suit::Club, value: Rank(3) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Spade, value: Rank(7) },
                Card { suit: Suit::Spade, value: Rank(JACK) },
                Card { suit: Suit::Diamond, value: Rank(JACK) },
                Card { suit: Suit::Club, value: Rank(ACE) },
                Card { suit: Suit::Diamond, value: Rank(2) },
                Card { suit: Suit::Spade, value: Rank(3) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Diamond, value: Rank(3) },
                Card { suit: Suit::Club, value: Rank(2) },
                Card { suit: Suit::Diamond, value: Rank(8) },
                Card { suit: Suit::Club, value: Rank(8) },
                Card { suit: Suit::Diamond, value: Rank(KING) },
                Card { suit: Suit::Diamond, value: Rank(QUEEN) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Spade, value: Rank(10) },
                Card { suit: Suit::Heart, value: Rank(4) },
                Card { suit: Suit::Heart, value: Rank(2) },
                Card { suit: Suit::Club, value: Rank(10) },
                Card { suit: Suit::Club, value: Rank(4) },
                Card { suit: Suit::Spade, value: Rank(8) },
            ]),
            Cascade(vec![
                Card { suit: Suit::Heart, value: Rank(QUEEN) },
                Card { suit: Suit::Diamond, value: Rank(4) },
                Card { suit: Suit::Spade, value: Rank(5) },
                Card { suit: Suit::Diamond, value: Rank(7) },
                Card { suit: Suit::Diamond, value: Rank(9) },
                Card { suit: Suit::Heart, value: Rank(KING) },
            ]),
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
