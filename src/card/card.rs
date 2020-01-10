use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use super::{parse_rank, Rank, Suit, ACE, JACK, KING, QUEEN};

/// A regular expression that matches cards.
pub const CARD_PATTERN: &str = r"(?x)
    (?P<rank>10|11|12|13|ace|jack|queen|king|[atjqk1-9])
    \s*(of)?\s*
    (?P<suit>c(lub(s)?)?|s(pade(s)?)?|h(eart(s)?)?|d(iamond(s)?)?)
";

/// Represents one card in the game.
///
/// Cards can be parsed from Strings or `&str`s.
/// The formats for this are the same formats used by Display and Debug.
/// See the description for `FromStr` below for details.
///
/// # Examples
///
/// Parsing cards from strings:
/// ```
/// # use freecell::Card;
/// use freecell::{ACE, JACK, KING, QUEEN};
/// use freecell::Suit::{Club, Diamond, Heart, Spade};
///
/// // Short representation used by Debug
/// assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), "AD".parse());
/// assert_eq!(Ok(Card { suit: Club, rank: 4 }), "4C".parse());
/// assert_eq!(Ok(Card { suit: Diamond, rank: 10 }), "TD".parse());
/// assert_eq!(Ok(Card { suit: Club, rank: JACK }), "jc".parse());
/// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), "qS".parse());
/// assert_eq!(Ok(Card { suit: Heart, rank: KING }), "Kh".parse());
///
/// // Long representation used by Display
/// assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), "Jack of Diamonds".parse());
/// assert_eq!(Ok(Card { suit: Club, rank: 10 }), "10 oF cLuBs".parse());
/// ```
///
/// A formatted card can be converted back to the original card:
/// ```
/// # use freecell::{Card, ACE};
/// # use freecell::Suit::Spade;
/// let ace_of_spades = Card { suit: Spade, rank: ACE };
/// // Formatted using Display
/// assert_eq!(Ok(ace_of_spades), ace_of_spades.to_string().parse());
/// // Formatted using Debug
/// assert_eq!(Ok(ace_of_spades), format!("{:?}", ace_of_spades).parse());
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Card {
    #[allow(missing_docs)]
    pub suit: Suit,
    #[allow(missing_docs)]
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl FromStr for Card {
    type Err = String;

    /// Converts a `&str` to a `Card`.
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
    /// # use freecell::Card;
    /// use freecell::{ACE, JACK, KING, QUEEN};
    /// use freecell::Suit::{Club, Diamond, Heart, Spade};
    ///
    /// assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), "AD".parse());
    /// assert_eq!(Ok(Card { suit: Club, rank: 4 }), "4C".parse());
    /// assert_eq!(Ok(Card { suit: Diamond, rank: 10 }), "TD".parse());
    /// assert_eq!(Ok(Card { suit: Club, rank: JACK }), "jc".parse());
    /// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), "qS".parse());
    /// assert_eq!(Ok(Card { suit: Heart, rank: KING }), "Kh".parse());
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
    /// # use freecell::Card;
    /// use freecell::{JACK, QUEEN};
    /// use freecell::Suit::{Club, Diamond, Spade};
    ///
    /// assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), "Jack of Diamonds".parse());
    /// assert_eq!(Ok(Card { suit: Club, rank: 3 }), "3 of clubs".parse());
    /// assert_eq!(Ok(Card { suit: Spade, rank: QUEEN }), "12 oF sPaDeS".parse());
    /// ```
    fn from_str(string: &str) -> Result<Card, Self::Err> {
        lazy_static! {
            static ref CARD_RE: Regex = Regex::new(format!(r"(?i)^\s*{}\s*$", CARD_PATTERN).as_str()).unwrap();
        }

        match CARD_RE.captures(string) {
            Some(caps) => Ok(Card {
                suit: caps.name("suit").unwrap().as_str().parse()?,
                rank: parse_rank(caps.name("rank").unwrap().as_str())?
            }),
            None => Err(format!("Could not parse card: \"{}\"", string))
        }
    }
}
