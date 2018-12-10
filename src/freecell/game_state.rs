use super::{CardCollection, Cascades, Foundations, Freecells, Move};
use super::position::Position;



#[derive(Debug, PartialEq)]
pub struct GameState {
    pub cascades: Cascades,
    pub foundations: Foundations,
    pub freecells: Freecells,
}

impl GameState {

    pub fn get_legal_moves(&self) -> Vec<(GameState, Move)> {
        // This is a temporary implementation.
        // My attempts to implement this nicely have been thwarted by rust's rules for creating
        // collections of traits (I tried to implement a method that returns a vector of
        // (CardCollection, Position), then iterate over it, try to pop cards and push them back
        // into a CardCollection. If both positions in a Move match, i.e. the card was pushed back
        // into the same CardCollection, the move was going to be filtered out.)
        // Unfortunately, I am stuck with this ugly mess for now:

        let mut legal_moves = Vec::new();

        // move from cascades ...
        for i in 0..self.cascades.len() {
            let (cascade, card) = match self.cascades[i].pop_card().pop() {
                Some((cascade, card)) => (cascade, card),
                None => continue,
            };

            // ... to other cascades
            for j in 0..self.cascades.len() {
                if i == j { continue }

                if let Ok(to_cascade) = self.cascades[j].add_card(card) {
                    let mut new_cascades = self.cascades.clone();
                    new_cascades[i] = cascade.clone();
                    new_cascades[j] = to_cascade;
                    legal_moves.push((
                        GameState {
                            cascades: new_cascades,
                            foundations: self.foundations.clone(),
                            freecells: self.freecells.clone(),
                        },
                        Move {
                            card,
                            from: Position::Cascade(i),
                            to: Position::Cascade(j),
                        }
                    ));
                }
            }

            // ... to foundations
            if let Ok(foundations) = self.foundations.add_card(card) {
                let mut new_cascades = self.cascades.clone();
                new_cascades[i] = cascade.clone();
                legal_moves.push((
                    GameState {
                        cascades: new_cascades,
                        foundations,
                        freecells: self.freecells.clone(),
                    },
                    Move {
                        card,
                        from: Position::Cascade(i),
                        to: Position::Foundations,
                    }
                ));
            }

            // ... to freecells
            if let Ok(freecells) = self.freecells.add_card(card) {
                let mut new_cascades = self.cascades.clone();
                new_cascades[i] = cascade;
                legal_moves.push((
                    GameState {
                        cascades: new_cascades,
                        foundations: self.foundations.clone(),
                        freecells,
                    },
                    Move {
                        card,
                        from: Position::Cascade(i),
                        to: Position::Freecells,
                    }
                ));
            }
        }


        // move from freecells ...
        let from_freecells = self.freecells.pop_card();
        for (freecells, card) in from_freecells {
            // ... to cascades
            for i in 0..self.cascades.len() {
                if let Ok(cascade) = self.cascades[i].add_card(card) {
                    let mut new_cascades = self.cascades.clone();
                    new_cascades[i] = cascade;
                    legal_moves.push((
                        GameState {
                            cascades: new_cascades,
                            foundations: self.foundations.clone(),
                            freecells: freecells.clone(),
                        },
                        Move {
                            card,
                            from: Position::Freecells,
                            to: Position::Cascade(i),
                        }
                    ));
                }
            }

            // ... to foundations
            if let Ok(foundations) = self.foundations.add_card(card) {
                legal_moves.push((
                    GameState {
                        cascades: self.cascades.clone(),
                        foundations,
                        freecells: freecells.clone(),
                    },
                    Move {
                        card,
                        from: Position::Freecells,
                        to: Position::Foundations,
                    }
                ));
            }
        }

        legal_moves
    }

    pub fn is_solved(&self) -> bool {
        self.foundations[0].len() == 13 &&
        self.foundations[1].len() == 13 &&
        self.foundations[2].len() == 13 &&
        self.foundations[3].len() == 13
    }
}
