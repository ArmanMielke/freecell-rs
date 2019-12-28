use freecell::Suit::Spade;
use freecell::{Card, CardCollection, Freecell};

#[test]
fn test_empty() {
    let freecell: Freecell = None;
    assert_eq!(freecell.pop_card(), Vec::new());
}

#[test]
fn test_full() {
    let card = Card { suit: Spade, rank: 2 };
    let freecell: Freecell = Some(card);
    assert_eq!(freecell.pop_card(), vec![(None, card)]);
}
