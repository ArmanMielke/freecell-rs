use std::collections::HashSet;
use std::iter::FromIterator;

use crate::{Card, CardCollection, Cascade, Foundations, Freecells, Move, Position, POSITIONS};
use super::GameState;

#[derive(Clone)]
enum AnyCardCollection {
    Cascade(usize, Cascade),
    Foundations(Foundations),
    Freecells(Freecells),
}

impl GameState {

    pub fn legal_moves(&self) -> Vec<(GameState, Move)> {
        let mut legal_moves = Vec::new();

        // for all combinations of source and target position (where the two are different),
        // try to move cards from one to the other
        for &from_position in positions_except_for(&Position::Foundations) {
            for (from_card_collection, card) in self.pop_card_at_position(from_position) {
                for &to_position in positions_except_for(&from_position) {
                    if let Ok(to_card_collection) = self.add_card_at_position(to_position, card) {
                        legal_moves.push((
                            self.construct_next_game_state(from_card_collection.clone(), to_card_collection),
                            Move { card, from: from_position, to: to_position }
                        ));
                    }
                }
            }
        }

        legal_moves
    }

    fn pop_card_at_position(&self, position: Position) -> Vec<(AnyCardCollection, Card)> {
        // calls pop_card() at the desired card collection and replaces the returned card
        // collections with AnyCardCollections
        match position {
            Position::Cascade(i) => self.cascades[i].pop_card().drain(..).map(
                |(cascade, card)| (AnyCardCollection::Cascade(i, cascade), card)
            ).collect(),
            Position::Foundations => self.foundations.pop_card().drain(..).map(
                |(foundations, card)| (AnyCardCollection::Foundations(foundations), card)
            ).collect(),
            Position::Freecells => self.freecells.pop_card().drain(..).map(
                |(freecells, card)| (AnyCardCollection::Freecells(freecells), card)
            ).collect(),
        }
    }

    fn add_card_at_position(&self, position: Position, card: Card) -> Result<AnyCardCollection, ()> {
        // calls add_card(card) at the desired card collection and turns the result into an AnyCardCollection
        match position {
            Position::Cascade(i) => Ok(AnyCardCollection::Cascade(i, self.cascades[i].add_card(card)?)),
            Position::Foundations => Ok(AnyCardCollection::Foundations(self.foundations.add_card(card)?)),
            Position::Freecells => Ok(AnyCardCollection::Freecells(self.freecells.add_card(card)?)),
        }
    }

    fn construct_next_game_state(&self, from_card_collection: AnyCardCollection, to_card_collection: AnyCardCollection) -> GameState {
        // creates a game state that is identical to the current one, except for the source and
        // target position of the moved card. Those are identical to from_card_collection and
        // to_card_collection, respectively.
        // TODO there must be a way that does not require cloning the parts of the game state that are overwritten anyway
        let mut next_game_state = self.clone();

        for card_collection in vec![from_card_collection, to_card_collection].drain(..) {
            match card_collection {
                AnyCardCollection::Cascade(i, cascade) => next_game_state.cascades[i] = cascade,
                AnyCardCollection::Foundations(foundations) => next_game_state.foundations = foundations,
                AnyCardCollection::Freecells(freecells) => next_game_state.freecells = freecells,
            }
        }

        next_game_state
    }

}

/// Returns all possible positions, except for the given one.
fn positions_except_for(position: &Position) -> HashSet<&Position> {
    let mut positions = HashSet::from_iter(&POSITIONS);
    positions.remove(position);
    positions
}
