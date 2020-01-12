use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, Foundations, ACE, JACK, KING, QUEEN};

#[test]
fn test_empty() {
    assert_eq!("".parse(), Ok(Foundations::new()));
}

#[test]
fn test_non_empty() {
    // short card representation
    // not all foundations filled
    assert_eq!(
        "6S".parse(),
        Ok(Foundations([
            Vec::new(),
            vec![
                Card { suit: Spade, rank: ACE },
                Card { suit: Spade, rank: 2 },
                Card { suit: Spade, rank: 3 },
                Card { suit: Spade, rank: 4 },
                Card { suit: Spade, rank: 5 },
                Card { suit: Spade, rank: 6 },
            ],
            Vec::new(),
            Vec::new(),
        ]))
    );

    // short card representation without spaces between cards
    // all foundations filled
    assert_eq!(
        "TD2HAC4S".parse(),
        Ok(Foundations([
            vec![
                Card { suit: Club, rank: ACE },
            ],
            vec![
                Card { suit: Spade, rank: ACE },
                Card { suit: Spade, rank: 2 },
                Card { suit: Spade, rank: 3 },
                Card { suit: Spade, rank: 4 },
            ],
            vec![
                Card { suit: Heart, rank: ACE },
                Card { suit: Heart, rank: 2 },
            ],
            vec![
                Card { suit: Diamond, rank: ACE },
                Card { suit: Diamond, rank: 2 },
                Card { suit: Diamond, rank: 3 },
                Card { suit: Diamond, rank: 4 },
                Card { suit: Diamond, rank: 5 },
                Card { suit: Diamond, rank: 6 },
                Card { suit: Diamond, rank: 7 },
                Card { suit: Diamond, rank: 8 },
                Card { suit: Diamond, rank: 9 },
                Card { suit: Diamond, rank: 10 },
            ],
        ]))
    );

    // long card representation
    assert_eq!(
        "King of Hearts  3 of Clubs".parse(),
        Ok(Foundations([
            vec![
                Card { suit: Club, rank: ACE },
                Card { suit: Club, rank: 2 },
                Card { suit: Club, rank: 3 },
            ],
            Vec::new(),
            vec![
                Card { suit: Heart, rank: ACE },
                Card { suit: Heart, rank: 2 },
                Card { suit: Heart, rank: 3 },
                Card { suit: Heart, rank: 4 },
                Card { suit: Heart, rank: 5 },
                Card { suit: Heart, rank: 6 },
                Card { suit: Heart, rank: 7 },
                Card { suit: Heart, rank: 8 },
                Card { suit: Heart, rank: 9 },
                Card { suit: Heart, rank: 10 },
                Card { suit: Heart, rank: JACK },
                Card { suit: Heart, rank: QUEEN },
                Card { suit: Heart, rank: KING },
            ],
            Vec::new(),
        ]))
    );
}

#[test]
fn test_invalid() {
    // invalid card
    assert_eq!(
        "KD XX 8H".parse::<Foundations>(),
        Err("Could not parse foundations: \"KD XX 8H\"".to_string())
    );

    // cards separated by commas
    assert_eq!(
        "JH, TD, 9H".parse::<Foundations>(),
        Err("Could not parse foundations: \"JH, TD, 9H\"".to_string())
    );

    // too many foundations
    assert_eq!(
        "AC 7H KD JS 5C".parse::<Foundations>(),
        Err("Could not parse foundations: \"AC 7H KD JS 5C\"".to_string())
    );

    // multiple foundations of the same suit
    assert_eq!(
        "6C 7S 9C".parse::<Foundations>(),
        Err(format!("Multiple foundations of suit {} specified", Club))
    );
}
