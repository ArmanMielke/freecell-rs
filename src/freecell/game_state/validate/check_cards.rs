use crate::freecell::Card;
use crate::freecell::Suit::{Club, Diamond, Heart, Spade};
use super::super::GameState;



/// Checks whether each possible card exists exactly once
pub fn check_cards(game_state: &GameState) -> Result<(), String> {
    let card_count = count_cards(game_state);

    for card_index in 0..card_count.len() {
        if card_count[card_index] != 1 {
            return Err(format!(
                "Card {} exists {} times, should exist once",
                card_from_index(card_index), card_count[card_index]
            ));
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
    card.suit as usize * 13 + card.rank as usize - 1
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
        rank: (index % 13) as u8 + 1,
    }
}