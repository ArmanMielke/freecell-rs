use freecell::Suit::{Club, Diamond, Spade};
use freecell::{Card, GameState, Move, Position, JACK, KING, QUEEN};

#[test]
fn test_all_move_types_allowed() {
    let game_state = GameState::from_file("test-inputs/every-move-type-possible.txt").unwrap();
    for (_, legal_move) in game_state.legal_moves() {
        assert!(game_state.allows_move(legal_move));
    }

    // for each of these move types there is a move where the card is in the
    // correct source position, but cannot be added to the target position:
    // - cascade -> cascade
    // - cascade -> foundations
    // - freecells -> cascade
    // - freecells -> foundations

    // cascade -> cascade
    let card = Card { suit: Spade, rank: QUEEN };
    assert_eq!(game_state.cascades[5].last().unwrap(), &card);
    assert!(!game_state.allows_move(
        Move { card, from: Position::Cascade(5), to: Position::Cascade(0) }
    ));

    // cascade -> foundations
    let card = Card { suit: Diamond, rank: QUEEN };
    assert_eq!(game_state.cascades[7].last().unwrap(), &card);
    assert!(!game_state.allows_move(
        Move { card, from: Position::Cascade(7), to: Position::Foundations }
    ));

    // freecells -> cascade
    assert!(!game_state.allows_move(
        Move {
            card: Card { suit: Club, rank: JACK },
            from: Position::Freecell(2),
            to: Position::Cascade(2),
        }
    ));

    // freecells -> foundations
    assert!(!game_state.allows_move(
        Move {
            card: Card { suit: Club, rank: KING },
            from: Position::Freecell(0),
            to: Position::Foundations,
        }
    ));
}
