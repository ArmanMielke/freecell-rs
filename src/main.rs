mod freecell;

fn main() {
    let game_state = freecell::GameState::create_empty();
    println!("{}", game_state.get_legal_moves());
}
