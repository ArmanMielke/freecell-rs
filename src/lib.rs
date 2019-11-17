// TODO: see https://github.com/garro95/priority-queue for tips on speeding the priority-queue up


pub mod game_state_parser;

mod freecell;
mod state_graph;

#[cfg(test)]
mod tests;



use crate::state_graph::StateGraph;
use crate::freecell::{GameState, Move};



pub fn solve(initial_state: GameState) -> Option<Vec<Move>> {
    let mut state_graph = StateGraph::new(initial_state);
    state_graph.dijkstra()
}
