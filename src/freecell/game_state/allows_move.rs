use super::super::{Card, CardCollection, Move, Position};
use super::GameState;

impl GameState {

    /// Returns `true` if the given move is legal in this game state, `false` otherwise.
    /// Moves that don't change the game state are treated as illegal.
    pub fn allows_move(&self, game_move: Move) -> bool {
        // moves that don't change the game state are treated as illegal
        if game_move.from == game_move.to {
            return false;
        }

        // check whether the card can be removed from position game_move.from
        let can_pop_card = match game_move.from {
            Position::Cascade(i) => can_pop_from_collection(game_move.card, &self.cascades[i]),
            Position::Foundations => false,
            Position::Freecells => can_pop_from_collection(game_move.card, &self.freecells),
        };
        if !can_pop_card { return false; }

        // check whether the card can be added to position game_move.to
        let can_add_card = match game_move.to {
            Position::Cascade(i) => self.cascades[i].add_card(game_move.card).is_ok(),
            Position::Foundations => self.foundations.add_card(game_move.card).is_ok(),
            Position::Freecells => self.freecells.add_card(game_move.card).is_ok(),
        };
        can_add_card
    }

}

fn can_pop_from_collection<C: CardCollection>(card: Card, collection: &C) -> bool {
    for (_, popped_card) in collection.pop_card() {
        if card == popped_card { return true; }
    }
    false
}
