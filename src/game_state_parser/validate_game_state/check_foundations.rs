use crate::freecell::{Foundation, FoundationsTrait, GameState};
use crate::freecell::card::Suit;
use crate::freecell::card::Suit::*;



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
        if card.value != 1 + i as u8 {
            return Err(err_invgs_foundation_wrong_order!(suit, card.value, i + 1))
        }
    }
    Ok(())
}
