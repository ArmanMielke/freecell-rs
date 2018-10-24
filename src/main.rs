mod freecell;
mod game_state_parser;

fn main() {
    let game_state = game_state_parser::parse_file();
    println!("{}", game_state.get_legal_moves());
}
