use priority_queue::PriorityQueue;

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use crate::freecell::{Move, GameState, GameStateId};
use super::node::Node;



type Distance = Reverse<i32>;


// a graph in which nodes are expanded lazily once they are visited
pub struct StateGraph {
    initial_node_id: GameStateId,
    // nodes should only be added, they should never be removed
    // => a node that has been added once stays here forever
    nodes: HashMap<GameStateId, Node>,
}


impl StateGraph {

    pub fn new(initial_state: GameState) -> StateGraph {
        let initial_node = Node::new(initial_state);
        let initial_node_id = initial_node.id;
        let mut nodes = HashMap::new();
        nodes.insert(initial_node_id, initial_node);

        StateGraph {
            initial_node_id,
            nodes,
        }
    }

    pub(super) fn add_node(&mut self, game_state: GameState) {
        // only add the node if it has not been added before
        let node_has_been_added_before = { self.nodes.contains_key(&game_state.id()) };
        if !node_has_been_added_before {
            let node = Node::new(game_state);
            self.nodes.insert(node.id, node);
        }
    }

    pub fn dijkstra(&mut self) -> Option<Vec<Move>> {
        let mut visited: HashSet<GameStateId> = HashSet::new();
        // stores unvisited nodes. the nodes with the lowest tentative distance have the highest priority
        let mut priority_queue: PriorityQueue<GameStateId, Distance> = PriorityQueue::new();
        // stores tentative predecessors for each node
        // all nodes that are either in visited or in the priority queue must have a predecessor
        let mut predecessors: HashMap<GameStateId, GameStateId> = HashMap::new();

        priority_queue.push(self.initial_node_id, Reverse(0));

        while let Some((game_state_id, distance)) = priority_queue.pop() {
            visited.insert(game_state_id);

            // extract the node temporarily
            let mut node = self.nodes.remove(&game_state_id).unwrap();

            if node.is_goal_state() {
                return Some(self.construct_solution_path(predecessors, node));
            }

            for (_, neighbour_id) in node.get_edges(self).iter() {
                let neighbours_current_distance = priority_queue.get_priority(neighbour_id);
                let distance_via_current_node = Reverse(distance.0 + 1);
                if !visited.contains(neighbour_id)
                    && (
                        neighbours_current_distance.is_none()
                        || neighbours_current_distance.unwrap() < &distance_via_current_node
                    )
                {
                    priority_queue.push(*neighbour_id, distance_via_current_node);
                    predecessors.insert(*neighbour_id, game_state_id);
                }
            }

            // re-insert the node
            self.nodes.insert(node.id, node);
        }

        None
    }

    // this removes nodes from the graph and thus destroys it (could probably be fixed if necessary)
    fn construct_solution_path(&mut self, predecessors: HashMap<GameStateId, GameStateId>, goal_node: Node) -> Vec<Move> {
        println!("Solution found!");
        let mut moves = Vec::new();
        let mut current_node = goal_node;

        while let Some(predecessor_id) = predecessors.get(&current_node.id) {
            let mut predecessor = self.nodes.remove(predecessor_id).unwrap();

            {
                // find the edge that goes from the predecessor to the current node
                let predecessor_edges = predecessor.get_edges(self);
                let edge = predecessor_edges.iter().find(
                    // predicate is true for edges that end in current_node
                    |&&(_, node_id)| node_id == current_node.id
                ).unwrap();

                // prepend the move from that edge to the list of moves
                moves.insert(0, edge.0.clone());
            }

            current_node = predecessor;
        }

        moves
    }
}
