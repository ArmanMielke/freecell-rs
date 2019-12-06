use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::{rank_from_string, Rank, Suit, ACE, JACK, KING, QUEEN};

/// Represents one card in the game.
///
/// Cards can be converted from Strings or `&str`s.
/// The formats for this are the same formats used by Display and Debug.
/// See the descriptions for `TryFrom<String>` and `TryFrom<&str>` below for details.
///
/// # Examples
///
/// Parsing cards from strings:
/// ```
/// use std::convert::TryFrom;
/// # use freecell::Card;
/// use freecell::{ACE, JACK, KING, QUEEN};
/// use freecell::Suit::{Club, Diamond, Heart, Spade};
///
/// // Short representation used by Debug
/// assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), Card::try_from("AD"));
/// assert_eq!(Ok(Card { suit: Club, rank: 4 }), Card::try_from("4C"));
/// assert_eq!(Ok(Card { suit: Diamond, rank: 10 }), Card::try_from("TD"));
/// assert_eq!(Ok(Card { suit: Club, rank: JACK }), Card::try_from("jc"));
/// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), Card::try_from("qS"));
/// assert_eq!(Ok(Card { suit: Heart, rank: KING }), Card::try_from("Kh"));
///
/// // Long representation used by Display
/// assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), Card::try_from("Jack of Diamonds"));
/// assert_eq!(Ok(Card { suit: Club, rank: 10 }), Card::try_from("10 oF cLuBs"));
/// ```
///
/// A formatted card can be converted back to the original card:
/// ```
/// # use std::convert::TryFrom;
/// # use freecell::{Card, ACE};
/// # use freecell::Suit::Spade;
/// let ace_of_spades = Card { suit: Spade, rank: ACE };
/// // Formatted using Display
/// assert_eq!(Ok(ace_of_spades), Card::try_from(ace_of_spades.to_string()));
/// // Formatted using Debug
/// assert_eq!(Ok(ace_of_spades), Card::try_from(format!("{:?}", ace_of_spades)));
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rank_string = match self.rank {
            JACK => String::from("Jack"),
            QUEEN => String::from("Queen"),
            KING => String::from("King"),
            ACE => String::from("Ace"),
            _ => format!("{}", self.rank),
        };
        write!(f, "{} of {}s", rank_string, self.suit)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rank_string = match self.rank {
            JACK => String::from("J"),
            QUEEN => String::from("Q"),
            KING => String::from("K"),
            ACE => String::from("A"),
            10 => String::from("T"),
            _ => format!("{}", self.rank),
        };
        write!(f, "{}{:?}", rank_string, self.suit)
    }
}

// TODO [low priority] is there a solution for this:
// would like this to be
// impl<S: Into<String>> TryFrom<S> for Card
// but this doesn't work because of https://github.com/rust-lang/rust/issues/50133
impl TryFrom<String> for Card {
    type Error = String;

    /// Converts a String to a Card.
    ///
    /// The String must follow one of two formats.
    /// Both formats are case-insensitive.
    ///
    /// # The short format used by Debug
    ///
    /// The card is represented by a string of two characters.
    /// The first character denotes the card's rank:
    /// - 'A' or '1' - Ace
    /// - '2' - 2
    /// - ...
    /// - '9' - 9
    /// - 'T' - 10
    /// - 'J' - Jack
    /// - 'Q' - Queen
    /// - 'K' - King
    ///
    /// The second character denotes the suit:
    /// - 'C' - Club
    /// - 'S' - Spade
    /// - 'H' - Heart
    /// - 'D' - Diamond
    ///
    /// ## Examples
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use freecell::Card;
    /// use freecell::{ACE, JACK, KING, QUEEN};
    /// use freecell::Suit::{Club, Diamond, Heart, Spade};
    ///
    /// assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), Card::try_from("AD"));
    /// assert_eq!(Ok(Card { suit: Club, rank: 4 }), Card::try_from("4C"));
    /// assert_eq!(Ok(Card { suit: Diamond, rank: 10 }), Card::try_from("TD"));
    /// assert_eq!(Ok(Card { suit: Club, rank: JACK }), Card::try_from("jc"));
    /// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), Card::try_from("qS"));
    /// assert_eq!(Ok(Card { suit: Heart, rank: KING }), Card::try_from("Kh"));
    /// ```
    ///
    /// # The long format used by Display
    ///
    /// The card is represented by a string of the form "\<rank> of \<suit>s", where
    /// \<rank> can be the rank's number or its name.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use freecell::Card;
    /// use freecell::{JACK, QUEEN};
    /// use freecell::Suit::{Club, Diamond, Spade};
    ///
    /// assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), Card::try_from("Jack of Diamonds"));
    /// assert_eq!(Ok(Card { suit: Club, rank: 3 }), Card::try_from("3 of clubs"));
    /// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), Card::try_from("12 oF sPaDeS"));
    /// ```
    fn try_from(string: String) -> Result<Card, Self::Error> {
        // TODO [v1] use regex
        let string = string.trim();

        if string.len() == 2 {
            // string uses the short Debug format
            // the first character is the rank, the second character is the suit
            let suit = Suit::try_from(&string[1..2])?;
            let rank = rank_from_string(&string[0..1])?;
            Ok(Card { suit, rank })
        } else {
            // string seems to use the long Display format
            // the first word is the rank, the second word is the suit
            let string: Vec<&str> = string.split(' ').collect();
            let suit = Suit::try_from(*string.last().unwrap())?;
            let rank = rank_from_string(string[0])?;
            Ok(Card { suit, rank })
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    /// Converts a `&str` to a Card.
    ///
    /// See the description of `TryFrom<String>` for details.
    fn try_from(string: &str) -> Result<Card, Self::Error> {
        Card::try_from(string.to_string())
    }
}
