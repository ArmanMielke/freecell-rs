use freecell::Suit::{Club, Diamond, Heart, Spade};
use freecell::{Card, GameState, Move, Position, JACK, KING, QUEEN};

#[test]
fn test_hard_solved_to_6() {
    let game_state = GameState::from_file("test-inputs/hard-solved-to-6.txt").unwrap();
    for (_, legal_move) in game_state.legal_moves() {
        assert!(game_state.allows_move(legal_move));
    }

    ////////////////////////////
    // Card cannot be removed //
    ////////////////////////////

    // card cannot be removed from the cascade (but it can be added to the destination)
    let card = Card { suit: Heart, rank: 7 };
    assert!(game_state.cascades[2].contains(&card));
    assert!(!game_state.allows_move(
        Move { card, from: Position::Cascade(2), to: Position::Foundations }
    ));
    // this is illegal because there is another card on top of the 7 of Hearts

    // card cannot be removed from the foundations (but it can be added to the destination)
    assert!(!game_state.allows_move(
        Move {
            card: Card { suit: Club, rank: 6 },
            from: Position::Foundations,
            to: Position::Freecell(3),
        }
    ));
    // this is illegal because cards cannot be removed from the foundations

    // card is not in the freecells (but it can be added to the destination)
    let card = Card { suit: Club, rank: JACK };
    assert_eq!(game_state.cascades[1].last().unwrap(), &card);
    assert_eq!(game_state.cascades[5].last().unwrap(), &Card { suit: Diamond, rank: QUEEN });
    assert!(!game_state.allows_move(
        Move { card, from: Position::Freecell(0), to: Position::Cascade(5) }
    ));
    // moving the card from cascade 1 to cascade 5 would have been legal

    // to and from swapped, would otherwise be legal
    let card = Card { suit: Heart, rank: 10 };
    assert_eq!(game_state.cascades[0].last().unwrap(), &card);
    assert_eq!(game_state.cascades[1].last().unwrap(), &Card { suit: Club, rank: JACK });
    assert!(!game_state.allows_move(
        Move {
            card,
            from: Position::Cascade(1),
            to: Position::Cascade(0),
        }
    ));
    // moving the card from cascade 0 to cascade 1 would have been legal

    //////////////////////////
    // Card cannot be added //
    //////////////////////////

    // card cannot be added to the cascade (but it can be removed from its original position)
    let card = Card { suit: Spade, rank: 9};
    assert_eq!(game_state.cascades[3].last().unwrap(), &card);
    assert!(!game_state.allows_move(
        Move {
            card,
            from: Position::Cascade(3),
            to: Position::Cascade(6),
        }
    ));
    // this is illegal because there is an 8 at the top of cascade 6

    // card cannot be added to the foundations (but it can be removed from its original position)
    let card = Card { suit: Heart, rank: KING };
    assert_eq!(game_state.cascades[7].last().unwrap(), &card);
    assert!(!game_state.allows_move(
        Move {
            card,
            from: Position::Cascade(7),
            to: Position::Foundations,
        }
    ));
    // this is illegal because all the foundations only go up to 6
}
