use lazy_static::lazy_static;
use regex::Regex;

use crate::card::CARD_PATTERN;
use crate::{Card, CardCollection};

/// May hold any card.
///
/// # Examples
///
/// ```
/// # use freecell::Suit::{Diamond, Spade};
/// # use freecell::{Card, QUEEN};
/// use freecell::CardCollection;
///
/// let card = Card { suit: Diamond, rank: QUEEN };
///
/// // A card can be added to a freecell if it is empty
/// assert_eq!(
///     None.add_card(card),
///     Ok(Some(card))
/// );
/// // No card can be added to an occupied freecell
/// assert_eq!(
///     Some(Card { suit: Spade, rank: 4 }).add_card(card),
///     Err(())
/// );
///
/// // If the freecell is occupied, then a card can be removed
/// assert_eq!(
///     Some(card).pop_card(),
///     vec![(None, card)]
/// );
/// // No card can be removed if the freecell is empty
/// assert_eq!(
///     None.pop_card(),
///     Vec::new()
/// );
/// ```
pub type Freecell = Option<Card>;

impl CardCollection for Freecell {
    fn add_card(&self, card: Card) -> Result<Self, ()> {
        match self {
            Some(_) => Err(()),
            None => Ok(Some(card)),
        }
    }

    fn pop_card(&self) -> Vec<(Self, Card)> {
        match self {
            Some(card) => vec![(None, *card)],
            None => Vec::with_capacity(0),
        }
    }
}

/// May hold up to four arbitrary cards.
///
/// Can be parsed with [`parse_freecells`](fn.parse_freecells.html).
///
/// # Rules
///
/// There are four freecells.
/// Any card that can be moved from its current position can be moved to the freecells, as long as
/// no other card occupies that freecell.
/// Cards can be moved away from the freecells at any time.
///
/// # Examples
///
/// ```
/// # use freecell::Suit::{Club, Heart, Spade};
/// # use freecell::{parse_freecells, Card, CardCollection, ACE, KING};
/// let freecells = parse_freecells("empty AS empty 8C").unwrap();
/// assert_eq!(
///     freecells,
///     [
///         None,
///         Some(Card { suit: Spade, rank: ACE }),
///         None,
///         Some(Card { suit: Club, rank: 8 }),
///     ]
/// );
///
/// // A card can be added to the first freecell, since it is empty.
/// assert_eq!(
///     freecells[0].add_card(Card { suit: Heart, rank: KING }),
///     Ok(Some(Card { suit: Heart, rank: KING }))
/// );
/// // No card can be added to the second freecell, since it is occupied.
/// assert_eq!(
///     freecells[1].add_card(Card { suit: Heart, rank: KING }),
///     Err(())
/// );
///
/// // The card in the second freecell can be removed.
/// assert_eq!(
///     freecells[1].pop_card(),
///     vec![(None, Card { suit: Spade, rank: ACE })]
/// );
/// // No card can be removed from the first freecell, since it is empty.
/// assert_eq!(
///     freecells[0].pop_card(),
///     Vec::new()
/// );
/// ```
pub type Freecells = [Freecell; 4];

/// Converts a string to [`Freecells`](type.Freecells.html).
///
/// The input string should consist of up to four tokens, where each token consists of either a card
/// or "empty", case-insensitive.
/// The cards follow the format described in [`Card`](struct.Card.html)'s `FromStr`
/// implementation.
/// Tokens can optionally be separated by spaces.
///
/// # Examples
///
/// ```
/// # use freecell::Suit::{Club, Diamond, Heart, Spade};
/// # use freecell::{parse_freecells, Card, ACE, JACK};
/// assert_eq!(parse_freecells(""), Ok([None, None, None, None]));
///
/// assert_eq!(
///     parse_freecells("JH TD 9H"),
///     Ok([
///         Some(Card { suit: Heart, rank: JACK }),
///         Some(Card { suit: Diamond, rank: 10 }),
///         Some(Card { suit: Heart, rank: 9 }),
///         None,
///     ])
/// );
///
/// assert_eq!(
///     parse_freecells("empty AS empty 8C"),
///     Ok([
///         None,
///         Some(Card { suit: Spade, rank: ACE }),
///         None,
///         Some(Card { suit: Club, rank: 8 }),
///     ])
/// );
/// ```
pub fn parse_freecells<S: Into<String>>(string: S) -> Result<Freecells, String> {
    lazy_static! {
        static ref FREECELLS_RE: Regex = Regex::new(format!(r"(?i)^\s*(({}|empty)\s*){}$", CARD_PATTERN, "{0,4}").as_str()).unwrap();
        static ref FREECELL_RE: Regex = Regex::new(format!(r"(?i){}|empty", CARD_PATTERN).as_str()).unwrap();
    }

    let string = &string.into();
    if !FREECELLS_RE.is_match(string) {
        return Err(format!("Could not parse freecells: \"{}\"", string))
    }

    let mut freecells = [None, None, None, None];

    for (i, re_match) in FREECELL_RE.find_iter(string).enumerate() {
        // if the match cannot be parsed into a card, then it was "empty" => leave this freecell at None
        if let Ok(card) = re_match.as_str().parse() {
            freecells[i] = Some(card);
        }
    }

    Ok(freecells)
}
