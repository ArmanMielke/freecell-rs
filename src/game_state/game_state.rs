#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;

use crate::{Cascades, Foundations, Freecells, Card, Suit, ACE, Cascade};

pub type GameStateId = u64;

/// Represents the state of the board at one point during the course of a game.
///
/// The game state is defined by the cards that are in each of the three parts of the board, the
/// cascades, the foundations and the freecells.
///
/// A game state is valid if these conditions hold:
/// - Each of the 52 cards exists exactly once
/// - Each card on a foundation is of the correct suit
/// - The cards on each foundation are ordered correctly
///
/// # Examples
///
/// ```
/// // TODO [v1] Add code examples (once FromStr is implemented for GameState)
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameState {
    #[allow(missing_docs)]
    pub cascades: Cascades,
    #[allow(missing_docs)]
    pub foundations: Foundations,
    #[allow(missing_docs)]
    pub freecells: Freecells,
}

impl GameState {
    /// Parses the contents of a file into a GameState.
    ///
    /// This is equivalent to reading the contents of the file as `&str` and using that as input
    /// for `GameState::from_str`
    // TODO consider borrowing the file name: `file_name: &P` it doesn't really need to be consumed here, right?
    pub fn from_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(_) => return Err("File could not be read".to_string()),
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Err("File contents could not be read".to_string())
        }

        contents.parse()
    }

    /// Sets up an initial game state from a random "seed".
    ///
    /// # Examples
    ///
    /// ```
    /// use freecell::GameState;
    /// assert_eq!(&vec!["cascade: 7D TD TH KD 4C 4S JD",
    ///                  "cascade: AD 7S QC 5H QS TS KS",
    ///                  "cascade: 5C QD 3H 9S 9C 2H KC",
    ///                  "cascade: 3S AC 9D 3C 9H 5D 4H",
    ///                  "cascade: 5S 6D 6S 8S 7C JC",
    ///                  "cascade: 8C 8H 8D 7H 6H 6C",
    ///                  "cascade: 2D AS 3D 4D 2C JH",
    ///                  "cascade: AH KH TC JS 2S QH"].join("\n")
    ///            .parse::<GameState>().unwrap(),
    ///            &GameState::from_seed(617));
    /// ```
    pub fn from_seed(mut rand_seed: i32) -> GameState {
	let suits = vec![Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
	let mut lcg = || {
	    rand_seed = rand_seed.wrapping_mul(214013).wrapping_add(2531011)
		& 0x7fffffff;
	    rand_seed >> 16
	};
	let mut deck: Vec<Card> = (0..52)
	    .map(|i| {
		let r: u8 = (i / 4) as u8;
		Card{
		    suit: suits[i % 4],
		    rank: match r { 0 => ACE, _ => r + 1},
		}
	    })
	    .collect();
        let mut cards: Vec<Vec<Card>> = (0..8).map(|_| Vec::with_capacity(7)).collect();
	for i in 0..52 {
	    let which = lcg() as usize % deck.len(); // select a random card
	    cards[i % 8].push(deck.swap_remove(which)); // add to the next cascade
	}
	GameState{
	    cascades: [Cascade(cards[0].clone()), Cascade(cards[1].clone()),
		       Cascade(cards[2].clone()), Cascade(cards[3].clone()),
		       Cascade(cards[4].clone()), Cascade(cards[5].clone()),
		       Cascade(cards[6].clone()), Cascade(cards[7].clone())],
	    foundations: Foundations::new(),
	    freecells: [None, None, None, None],
	}
    }

    /// Returns `true` if all cards are on the foundations, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use freecell::GameState;
    /// assert!("foundations: KC KS KH KD".parse::<GameState>().unwrap().is_solved());
    /// assert!(
    ///     !"
    ///         foundations: KC KS KH QD
    ///         cascade: KD
    ///     ".parse::<GameState>().unwrap().is_solved()
    /// );
    /// ```
    pub fn is_solved(&self) -> bool {
        self.foundations.0[0].len() == 13 &&
        self.foundations.0[1].len() == 13 &&
        self.foundations.0[2].len() == 13 &&
        self.foundations.0[3].len() == 13
    }

    // TODO [v1] document
    pub fn id(&self) -> GameStateId {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
