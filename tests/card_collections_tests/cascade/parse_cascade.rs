use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{parse_cascade, Card, Cascade, ACE, KING, QUEEN};

#[test]
fn test_empty() {
    assert_eq!(parse_cascade(""), Ok(Cascade::new()));
}

#[test]
fn test_non_empty() {
    // short card representation
    assert_eq!(
        parse_cascade("9S 7H AC"),
        Ok(Cascade(vec![
            Card { suit: Spade, rank: 9 },
            Card { suit: Heart, rank: 7 },
            Card { suit: Club, rank: ACE },
        ]))
    );

    // short card representation without spaces between cards
    assert_eq!(
        parse_cascade("3D6CKCQH"),
        Ok(Cascade(vec![
            Card { suit: Diamond, rank: 3 },
            Card { suit: Club, rank: 6 },
            Card { suit: Club, rank: KING },
            Card { suit: Heart, rank: QUEEN },
        ]))
    );

    // long card representation
    assert_eq!(
        parse_cascade("10 of Hearts  4 of Clubs"),
        Ok(Cascade(vec![
            Card { suit: Heart, rank: 10 },
            Card { suit: Club, rank: 4 },
        ]))
    );
}

#[test]
fn test_invalid() {
    // invalid card
    assert_eq!(
        parse_cascade("KD XX 8H"),
        Err("Could not parse cascade: \"KD XX 8H\"".to_string())
    );

    // cards separated by commas
    assert_eq!(
        parse_cascade("JH, TD, 9H"),
        Err("Could not parse cascade: \"JH, TD, 9H\"".to_string())
    );
}
