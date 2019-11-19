use crate::freecell::{Foundation, FoundationsTrait, GameState, Suit};
use crate::freecell::Suit::{Club, Diamond, Heart, Spade};



/// Checks whether there is one foundation of each suit and whether each individual foundation is
/// correct.
pub fn check_foundations(game_state: &GameState) -> Result<(), String> {
    for suit in [Club, Spade, Heart, Diamond].iter() {
        check_foundation(&game_state.foundations.get_foundation(*suit), *suit)?;
    }
    Ok(())
}


/// Checks whether the cards in a foundation have the required suit and are in the correct order.
fn check_foundation(foundation: &Foundation, suit: Suit) -> Result<(), String> {
    for (i, card) in foundation.iter().enumerate() {
        if card.suit != suit {
            return Err(err_invgs_foundation_wrong_suit!(suit, card.suit, card))
        }
        if card.rank != 1 + i as u8 {
            return Err(err_invgs_foundation_wrong_order!(suit, card.rank, i + 1))
        }
    }
    Ok(())
}
