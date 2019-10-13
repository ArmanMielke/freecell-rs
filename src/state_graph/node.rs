use freecell::{Move, GameState, GameStateId};
use super::StateGraph;
use std::cell::{Cell, RefCell, Ref};


pub struct Node {
    pub id: GameStateId, // TODO this can be mutable, but shouldn't (use getter or something like that) is this field even necessary?
    game_state: GameState,
    /// Outgoing edges
    edges: RefCell<Vec<(Move, GameStateId)>>, // TODO use RefCell<Option<Vec<(Move, GameStateId)>>>, so the extra field "expanded" is not needed
    expanded: Cell<bool>,
}

impl Node {

    pub fn new(game_state: GameState) -> Node {
        Node {
            id: game_state.generate_id(),
            game_state,
            edges: RefCell::new(Vec::new()),
            expanded: Cell::new(false),
        }
    }

    /// Expands the node if it has not been expanded yet, potentially adding new nodes to the graph.
    /// It then returns outgoing edges.
    pub fn get_edges(&self, graph: &mut StateGraph /*TODO is there a solution where I don't need to pass the Graph?*/) -> Ref<Vec<(Move, GameStateId)>> {

        // "if self.expanded = false {...}". in either case, set the value to true
        if !self.expanded.replace(true) {

            // expand node, i.e. generate edges and add neighbours to the graph
            let moves = self.game_state.get_legal_moves();
            for (game_state, game_move) in moves {
                let id = game_state.generate_id();
                graph.add_node(game_state);
                self.edges.borrow_mut().push((game_move, id));
            }
        }

        self.edges.borrow()
    }

    pub fn is_goal_state(&self) -> bool {
        self.game_state.is_solved()
    }
}

