use freecell::{Move, GameState, GameStateId};
use super::node::Node;

use std::cell::{RefCell, Ref};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;


type Distance = Reverse<i32>;


// a graph in which nodes are expanded lazily once they are visited
pub struct StateGraph {
    // nodes should only be added, they should never be removed
    // => a node that has been added once stays here forever
    nodes: RefCell<HashMap<GameStateId, Node>>,
}


impl StateGraph {

    pub fn new(initial_state: GameState) -> StateGraph {
        let initial_node = Node::new(initial_state);
        let mut nodes = HashMap::new();
        nodes.insert(initial_node.id, initial_node);

        StateGraph {
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

    pub fn dijkstra(&self, starting_node_id: GameStateId) -> Option<Vec<Move>> {
        // TODO implement

        let mut visited: HashSet<GameStateId> = HashSet::new();
        let mut priority_queue: PriorityQueue<GameStateId, Distance> = PriorityQueue::new();
        priority_queue.push(starting_node_id, Reverse(0));

        while let Some((game_state_id, distance)) = priority_queue.pop() {
            visited.insert(game_state_id);

            let nodes = self.nodes.borrow();
            let node = nodes.get(&game_state_id).unwrap();

            if node.is_goal_state() {
                // TODO return solution
                unimplemented!()
            }


            for (game_move, neighbour_id) in node.get_edges(&self).iter() {

            }
        }

        None
    }

}
