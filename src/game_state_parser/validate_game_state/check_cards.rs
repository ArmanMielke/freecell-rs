use crate::freecell::{Card, GameState};
use crate::freecell::card::Suit::*;



/// Checks whether each possible card exists exactly once
pub fn check_cards(game_state: &GameState) -> Result<(), String> {
    let card_count = count_cards(game_state);

    for card_index in 0..card_count.len() {
        if card_count[card_index] != 1 {
            return Err(err_invgs_card_does_not_exist_exactly_once!(
                card_from_index(card_index),
                card_count[card_index]
            ))
        }
    }

    Ok(())
}


/// For each of the 52 possible cards, this function counts how many instances of this card exist.
fn count_cards(game_state: &GameState) -> [i32; 52] {
    let mut card_count = [0; 52];

    for cascade in game_state.cascades.iter() {
        for card in cascade {
            card_count[card_index(card)] += 1;
        }
    }

    for foundation in game_state.foundations.iter() {
        for card in foundation {
            card_count[card_index(card)] += 1;
        }
    }

    for card in game_state.freecells.iter() {
        card_count[card_index(card)] += 1;
    }

    card_count
}


fn card_index(card: &Card) -> usize {
    card.suit as usize * 13 + card.value as usize - 1
}


fn card_from_index(index: usize) -> Card {
    Card {
        suit: match index / 13 {
            0 => Club,
            1 => Spade,
            2 => Heart,
            3 => Diamond,
            _ => unreachable!("invalid card index: {}", index)
        },
        value: (index % 13) as u8 + 1,
    }
}
