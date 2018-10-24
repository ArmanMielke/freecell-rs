use freecell::*;
use freecell::Suit::*;

pub fn parse_file(/* TODO: Path to input file */) -> GameState {

    // TODO actually parse this
    let cascades = [
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
        Cascade(Vec::new()),
    ];

    // TODO actually parse this
    let foundations = [
        Foundation{
            suit: Club,
            cards: Vec::new(),
        },
        Foundation{
            suit: Spade,
            cards: Vec::new(),
        },
        Foundation{
            suit: Heart,
            cards: Vec::new(),
        },
        Foundation{
            suit: Diamond,
            cards: Vec::new(),
        },
    ];

    // TODO actually parse this
    let freecells = [None, None, None, None];

    GameState {cascades, foundations, freecells}
}