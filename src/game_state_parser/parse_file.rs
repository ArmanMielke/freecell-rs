use freecell::*;
use freecell::Suit::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

const FOUNDATIONS: &str = "foundations:";
const CASCADE: &str = "cascade:";
const FREECELLS: &str = "freecells:";



pub fn parse_file<P: AsRef<Path>>(file_name: P) -> GameState {
    let lines = read_file_as_lines(file_name);

    for line in lines {
        let asdf = line?;
    }

    // <random code fragment>

    // skip empty lines

    let first_token_in_line = "";

    match first_token_in_line {
        FOUNDATIONS => println!("parse foundations"),
        CASCADE => println!("parse cascade"),
        FREECELLS => println!("parse freecells"),
        _ => println!("ignore and print warning"),
    }

    let asdf: Vec<i8> = Vec::new();
    asdf.

    // </random code fragment>


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

fn read_file_as_lines<P: AsRef<Path>>(file_name: P) -> Lines<BufReader<File>> {
    // TODO: make sense of the ? operator
    let mut file = File::open(file_name)?;
    let buffered_reader = BufReader::new(file);
    buffered_reader.lines()
}
