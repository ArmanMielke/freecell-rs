use freecell::{Foundation, GameState};
use freecell::card::{Suit, Card};
use freecell::card::Suit::*;


/// Checks whether a game state is correct, i.e.
/// - each card exists exactly once
/// - each card on a foundation is of the correct suit
/// - the cards on a foundation are ordered correctly
pub fn validate_game_state(game_state: &GameState) -> Result<(), String> {
    check_cards(game_state)?;
    check_foundations(game_state)?;
    Ok(())
}

/// Checks whether each possible card exists exactly once
fn check_cards(game_state: &GameState) -> Result<(), String> {
    // there are 52 unique cards in a deck
    // this array is used to count how many of each card exist (should be one for each card)
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

    for freecell in game_state.freecells.iter() {
        if let Some(card) = freecell {
            card_count[card_index(card)] += 1;
        }
    }

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
            _ => panic!("invalid card index: {}", index)
        },
        value: (index % 13) as u8,
    }
}


/// Checks whether there is one foundation of each suit and whether each individual foundation is
/// correct.
fn check_foundations(game_state: &GameState) -> Result<(), String> {
    for suit in [Club, Spade, Heart, Diamond].iter() {
        check_foundation(&game_state.foundations[*suit as usize], *suit)?;
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
