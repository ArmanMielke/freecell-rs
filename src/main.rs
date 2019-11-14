// TODO there is some way to get rid of the extern crate declarations in rust 2018 edition
extern crate arrayvec;
extern crate priority_queue; // TODO: see https://github.com/garro95/priority-queue for tips on speeding it up


mod freecell;
mod game_state_parser;
mod state_graph;

#[cfg(test)]
mod tests;



use state_graph::StateGraph;
use freecell::{GameState, Move};



fn main() {
    let file_name = "TODO"; // TODO take file name from input
    let initial_state = game_state_parser::parse_file(file_name).unwrap();
    let solution = solve(initial_state);
    match solution {
        Some(moves) => moves.iter().for_each(
            |game_move| print!("{}", game_move)
        ),
        None => print!("No solution found"),
    };
}

fn solve(initial_state: GameState) -> Option<Vec<Move>> {
    let mut state_graph = StateGraph::new(initial_state);
    state_graph.dijkstra()
}
