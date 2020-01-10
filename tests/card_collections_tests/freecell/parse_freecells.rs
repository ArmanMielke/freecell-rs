use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{parse_freecells, Card, Freecells, ACE, JACK, KING, QUEEN};

#[test]
fn test_empty() {
    let freecells: Result<Freecells, String> = Ok([None, None, None, None]);
    assert_eq!(parse_freecells(""), freecells);
    assert_eq!(parse_freecells("EmPtY EMPTY"), freecells);
    assert_eq!(parse_freecells("emptyemptyemptyempty"), freecells);
}

#[test]
fn test_partly_filled() {
    // short card representation
    assert_eq!(
        parse_freecells("JH TD 9H"),
        Ok([
            Some(Card { suit: Heart, rank: JACK }),
            Some(Card { suit: Diamond, rank: 10 }),
            Some(Card { suit: Heart, rank: 9 }),
            None,
        ])
    );

    // long card representation
    assert_eq!(
        parse_freecells("queenofhearts2ofclubs"),
        Ok([
            Some(Card { suit: Heart, rank: QUEEN }),
            Some(Card { suit: Club, rank: 2 }),
            None,
            None,
        ])
    )
}

#[test]
fn test_explicit_empty_slots() {
    // empty slot in the beginning
    assert_eq!(
        parse_freecells("empty empty 8C"),
        Ok([
            None,
            None,
            Some(Card { suit: Club, rank: 8 }),
            None,
        ])
    );

    // empty slots in the middle
    assert_eq!(
        parse_freecells("2H empty empty 7D"),
        Ok([
            Some(Card { suit: Heart, rank: 2 }),
            None,
            None,
            Some(Card { suit: Diamond, rank: 7 }),
        ])
    );

    // empty slot in the end
    assert_eq!(
        parse_freecells("5C 6C empty empty"),
        Ok([
            Some(Card { suit: Club, rank: 5 }),
            Some(Card { suit: Club, rank: 6 }),
            None,
            None,
        ])
    );
}

#[test]
fn test_full() {
    // short card representation
    assert_eq!(
        parse_freecells("AC7SKH1D"),
        Ok([
            Some(Card { suit: Club, rank: ACE }),
            Some(Card { suit: Spade, rank: 7 }),
            Some(Card { suit: Heart, rank: KING }),
            Some(Card { suit: Diamond, rank: ACE }),
        ])
    );

    // long card representation
    assert_eq!(
        parse_freecells("Ace of Diamonds  10 of Spades  Jack of Hearts  3 of Clubs"),
        Ok([
            Some(Card { suit: Diamond, rank: ACE }),
            Some(Card { suit: Spade, rank: 10 }),
            Some(Card { suit: Heart, rank: JACK }),
            Some(Card { suit: Club, rank: 3 }),
        ])
    )
}
