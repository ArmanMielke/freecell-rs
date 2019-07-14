// TODO there is some way to get rid of the extern crate declarations in rust 2018 edition
extern crate arrayvec;
extern crate priority_queue; // TODO: see https://github.com/garro95/priority-queue for tips on speeding it up

mod freecell;
mod game_state_parser;
mod state_graph;

use state_graph::StateGraph;

fn main() {
    let file_name = "TODO"; // TODO take file name from input
    let initial_state = game_state_parser::parse_file(file_name).unwrap();
    let state_graph = StateGraph::new(initial_state);
    let solution = state_graph.dijkstra();
    match solution {
        Some(moves) => moves.iter().for_each(
            |game_move| print!("{}", game_move)
        ),
        None => print!("No solution found"),
    };
}
