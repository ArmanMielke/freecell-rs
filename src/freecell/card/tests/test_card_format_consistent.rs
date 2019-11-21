use std::convert::TryFrom;

use super::super::{Card, ACE, JACK, KING, QUEEN};
use super::super::Suit::{Club, Diamond, Heart, Spade};



const CARDS: &'static [Card] = &[
    Card { suit: Heart, rank: ACE},
    Card { suit: Diamond, rank: 5},
    Card { suit: Spade, rank: 10},
    Card { suit: Heart, rank: JACK},
    Card { suit: Spade, rank: QUEEN},
    Card { suit: Club, rank: KING},
];

#[test]
fn test_display() {
    for card in CARDS {
        assert_eq!(Ok(*card), Card::try_from(card.to_string()));
    }
}

#[test]
fn test_debug() {
    for card in CARDS {
        assert_eq!(Ok(*card), Card::try_from(format!("{:?}", card)));
    }
}
