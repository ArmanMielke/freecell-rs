use crate::freecell::{Move, GameState, GameStateId};
use super::StateGraph;



pub struct Node {
    pub id: GameStateId, // TODO this can be mutable, but shouldn't (use getter or something like that) is this field even necessary?
    game_state: GameState,
    /// Outgoing edges
    edges: Vec<(Move, GameStateId)>, // TODO use Option<Vec<(Move, GameStateId)>>, so the extra field "expanded" is not needed (is that possible?)
    expanded: bool,
}

impl Node {

    pub fn new(game_state: GameState) -> Node {
        Node {
            id: game_state.generate_id(),
            game_state,
            edges: Vec::new(),
            expanded: false,
        }
    }

    /// Expands the node if it has not been expanded yet, potentially adding new nodes to the graph.
    /// It then returns outgoing edges.
    pub fn get_edges(&mut self, graph: &mut StateGraph /*TODO is there a solution where I don't need to pass the Graph?*/) -> &Vec<(Move, GameStateId)> {
        if !self.expanded {
            self.expanded = true;

            // expand node, i.e. generate edges and add neighbours to the graph
            let moves = self.game_state.legal_moves();
            for (game_state, game_move) in moves {
                let id = game_state.generate_id();
                graph.add_node(game_state);
                self.edges.push((game_move, id));
            }
        }

        &self.edges
    }

    pub fn is_goal_state(&self) -> bool {
        self.game_state.is_solved()
    }
}

