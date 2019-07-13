use freecell::{Move, GameState, GameStateId};
use super::StateGraph;


pub struct Node {
    pub id: GameStateId, // TODO this can be mutable, but shouldn't (use getter or something like that)
    game_state: GameState,
    /// Outgoing edges
    edges: Vec<(Move, GameStateId)>,
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

    pub fn expand(&mut self, graph: &mut StateGraph /*TODO is there a solution where I don't need to pass the Graph?*/) {
        if self.expanded { return; }
        self.expanded = true;

        let moves = self.game_state.get_legal_moves();
        for (game_state, game_move) in moves {
            let id = game_state.generate_id();
            graph.add_node(game_state);
            self.edges.push((game_move, id));
        }
    }

    pub fn is_goal_state(&self) -> bool {
        self.game_state.is_solved()
    }
}

