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

use std::env;



// TODO test main()
fn main() {
    // TODO handle Ctrl+C: https://rust-lang-nursery.github.io/cli-wg/in-depth/signals.html (not sure whether this is necessary)

    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) {
        println!("Usage: {} <problem file>", args[0]);
        return;
    }
    let input_file_name = args.into_iter().nth(1).unwrap();

    // parse input file
    let initial_state = match game_state_parser::parse_file(input_file_name) {
        Ok(game_state) => game_state,
        Err(msg) => {
            eprintln!("ERROR: {}", msg);
            return;
        },
    };
    println!("{}", initial_state);
    println!();

    // solve problem
    let solution = solve(initial_state);

    // print solution
    if let Some(moves) = solution {
        for (move_number, game_move) in moves.iter().enumerate() {
            println!("{}: {}", move_number + 1, game_move)
        }
    } else {
        println!("No solution found")
    }
}

fn solve(initial_state: GameState) -> Option<Vec<Move>> {
    let mut state_graph = StateGraph::new(initial_state);
    state_graph.dijkstra()
}
