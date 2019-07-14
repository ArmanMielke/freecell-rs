use freecell::{Move, GameState, GameStateId};
use super::node::Node;

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use priority_queue::PriorityQueue;


type Distance = Reverse<i32>;


// a graph in which nodes are expanded lazily once they are visited
pub struct StateGraph {
    initial_node_id: GameStateId,
    // nodes should only be added, they should never be removed
    // => a node that has been added once stays here forever
    nodes: RefCell<HashMap<GameStateId, Node>>,
}


// TODO replace uses of GameStateIds with references to Nodes as much as possible
impl StateGraph {

    pub fn new(initial_state: GameState) -> StateGraph {
        let initial_node = Node::new(initial_state);
        let initial_node_id = initial_node.id;
        let mut nodes = HashMap::new();
        nodes.insert(initial_node_id, initial_node);

        StateGraph {
            initial_node_id,
            nodes: RefCell::new(nodes),
        }
    }

    pub(super) fn add_node(&self, game_state: GameState) {
        // only add the node if it has not been added before
        if !self.nodes.borrow().contains_key(&game_state.generate_id()) {
            let node = Node::new(game_state);
            self.nodes.borrow_mut().insert(node.id, node);
        }
    }

    pub fn dijkstra(&self) -> Option<Vec<Move>> {
        let nodes = self.nodes.borrow();

        // stores visited node as well as their predecessor in the shortest path (predecessor is None for the first node)
        let mut visited: HashMap<GameStateId, Option<GameStateId>> = HashMap::new();
        // stores unvisited nodes with their tentative predecessor and distance
        let mut priority_queue: PriorityQueue<(GameStateId, Option<GameStateId>), Distance> = PriorityQueue::new();
        priority_queue.push((self.initial_node_id, None), Reverse(0));

        while let Some(((game_state_id, predecessor_id), distance)) = priority_queue.pop() {
            visited.insert(game_state_id, predecessor_id);

            let node = nodes.get(&game_state_id).unwrap();

            if node.is_goal_state() {
                return Some(self.construct_solution_path(visited, node.id));
            }


            for (_, neighbour_id) in node.get_edges(&self).iter() {
                unimplemented!()
            }
        }

        None
    }

    /// # Parameters
    /// - visited: A map with each visited node and their predecessor in the shortest path.
    ///            The predecessor is None for the starting node. All GameStateIds must be in self.nodes
    /// - goal_node_id: The GameStateId of the last node. This must be a key in visited.
    fn construct_solution_path(&self, visited: HashMap<GameStateId, Option<GameStateId>>, goal_node_id: GameStateId) -> Vec<Move> {
        print!("Solution found!");
        let nodes = self.nodes.borrow();
        let mut moves = Vec::new();
        let mut current_node = nodes.get(&goal_node_id).unwrap();

        while let Some(predecessor_id) = visited.get(&current_node.id).unwrap() {
            let predecessor = nodes.get(predecessor_id).unwrap();

            // find the edge that goes from the predecessor to the current node
            let predecessor_edges = predecessor.get_edges(self);
            let edge = predecessor_edges.iter().find(
                // predicate is true for edges that end in current_node
                |&&(_, node_id)| node_id == current_node.id
            ).unwrap();

            // prepend the move from that edge to the list of moves
            moves.insert(0, edge.0.clone());

            current_node = predecessor;
        }

        moves
    }
}
