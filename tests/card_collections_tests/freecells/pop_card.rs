use arrayvec::ArrayVec;

use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, CardCollection, Freecells};

#[test]
fn test_empty() {
    let freecells: Freecells = ArrayVec::new();
    assert_eq!(freecells.pop_card(), Vec::new());
}

#[test]
fn test_one_card() {
    let card = Card { suit: Club, rank: 8 };

    let mut freecells: Freecells = ArrayVec::new();
    freecells.push(card);

    assert_eq!(freecells.pop_card(), vec![(ArrayVec::new(), card)]);
}

#[test]
fn test_full() {
    let card_1 = Card { suit: Spade, rank: 2 };
    let card_2 = Card { suit: Diamond, rank: 3 };
    let card_3 = Card { suit: Heart, rank: 4 };
    let card_4 = Card { suit: Club, rank: 5 };

    let freecells: Freecells = ArrayVec::from([card_1, card_2, card_3, card_4]);

    let mut expected_freecells_1 = freecells.clone();
    expected_freecells_1.swap_remove(0);
    let mut expected_freecells_2 = freecells.clone();
    expected_freecells_2.swap_remove(1);
    let mut expected_freecells_3 = freecells.clone();
    expected_freecells_3.swap_remove(2);
    let mut expected_freecells_4 = freecells.clone();
    expected_freecells_4.swap_remove(3);
    let expected = vec![
        (expected_freecells_1, card_1),
        (expected_freecells_2, card_2),
        (expected_freecells_3, card_3),
        (expected_freecells_4, card_4),
    ];

    assert_eq!(freecells.pop_card(), expected);
}
