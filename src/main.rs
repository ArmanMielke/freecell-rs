extern crate petgraph;
extern crate arrayvec;

mod freecell;
mod game_state_parser;
mod state_graph;

use state_graph::StateGraph;

fn main() {
    let file_name = "TODO"; // TODO take file name from input
    let initial_state = game_state_parser::parse_file(file_name).unwrap();
    let starting_id = initial_state.generate_id();
    let mut state_graph = StateGraph::new(initial_state);
    state_graph.dijkstra(starting_id);
    // TODO print solution
}
