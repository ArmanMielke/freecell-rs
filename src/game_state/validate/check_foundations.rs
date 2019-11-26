use super::super::GameState;
use crate::Suit::{Club, Diamond, Heart, Spade};
use crate::{Foundation, Suit};

/// Checks whether there is one foundation of each suit and whether each individual foundation is
/// correct.
pub fn check_foundations(game_state: &GameState) -> Result<(), String> {
    for suit in [Club, Spade, Heart, Diamond].iter() {
        check_foundation(&game_state.foundations.foundation(*suit), *suit)?;
    }
    Ok(())
}

/// Checks whether the cards in a foundation have the required suit and are in the correct order.
fn check_foundation(foundation: &Foundation, suit: Suit) -> Result<(), String> {
    for (i, card) in foundation.iter().enumerate() {
        if card.suit != suit {
            return Err(format!(
                "Foundation of suit {} contains card of suit {}: {}",
                suit, card.suit, card
            ));
        }
        if card.rank != 1 + i as u8 {
            return Err(format!(
                "Foundation of suit {} contains card of rank {} at position {}",
                suit, card.rank, i + 1
            ));
        }
    }
    Ok(())
}
