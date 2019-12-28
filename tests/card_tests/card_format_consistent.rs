use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, ACE, JACK, KING, QUEEN};

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
        assert_eq!(Ok(*card), card.to_string().parse());
    }
}

#[test]
fn test_debug() {
    for card in CARDS {
        assert_eq!(Ok(*card), format!("{:?}", card).parse());
    }
}
