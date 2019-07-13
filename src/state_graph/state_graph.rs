use freecell::{Move, GameState, GameStateId};
use super::node::Node;


// a graph in which nodes are expanded lazily once they are visited
pub struct StateGraph {
    // nodes should only be added, they should never be removed
    // => a node that has been added once stays here forever
    nodes: Vec<Node>, // TODO alternative: use hash map with ids as keys and nodes as values
}


impl StateGraph {

    pub fn new(initial_state: GameState) -> StateGraph {
        let initial_node = Node::new(initial_state);
        StateGraph {
            nodes: vec![initial_node],
        }
    }

    // TODO restrict visibility
    pub fn add_node(&mut self, game_state: GameState) {
        // only add the node if it has not been added before
        match self.get_node(game_state.generate_id()) {
            Some(_) => return,
            None => self.nodes.push(Node::new(game_state)),
        }
    }

    fn get_node(&self, id: GameStateId) -> Option<&Node> {
        for node in &self.nodes {
            if node.id == id {
                return Some(node)
            }
        }

        None
    }

    pub fn dijkstra(&mut self, starting_node_id: GameStateId) -> Vec<Move> {
        // TODO implement
        unimplemented!()
    }

}

