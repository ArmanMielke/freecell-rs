use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

use crate::card::CARD_PATTERN;
use crate::{Card, CardCollection, Suit, ACE};

//const FOUNDATION_MAX_SIZE: usize = 13;

/// A stack of cards of one suit, ordered from Ace upwards.
///
/// The end of the `Vec` is the top of the stack.
///
/// See struct Foundations.
pub type Foundation = Vec<Card>;

/// Four stacks of cards, where each stack contains only cards of one suit, going from Ace upwards.
///
/// # Rules
///
/// There exists one foundation for each of the four suits.
/// Cards can only be put on the foundation corresponding to their suit.
///
/// An Ace can be put on the foundation of the appropriate suit iff there is no other card on that
/// foundation.
/// This is always the case in a valid game state.
///
/// Any other card *c* can be put on the foundation of the appropriate suit iff the top card on that
/// foundation has a rank exactly one lower than card *c*.
///
/// This means a foundation can hold at most 13 cards:
/// the cards from Ace to King of one suit in order of their ranks, the Ace being on the bottom and
/// the King being on top.
/// The game ends in a victory when all four foundations reach this state.
///
/// Once a card is on a foundation, it can never be removed.
///
/// # Usage
///
/// The position of the Foundation in the array determines which suit it holds.
/// For example, `foundations[1]` may only hold spade cards, since `Suit::Spade` equals 1.
/// For a given suit the corresponding foundation can be accessed with
/// `foundations.foundation(suit)`.
///
/// # Examples
///
/// ```
/// // TODO [v1] Add code examples
/// ```
#[derive(Clone, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Foundations(pub [Foundation; 4]);

impl Foundations {
    /// Creates empty foundations.
    pub fn new() -> Foundations {
        Foundations([Vec::new(), Vec::new(), Vec::new(), Vec::new()])
    }

    /// Returns the foundation of the given suit.
    pub fn foundation(&self, suit: Suit) -> &Foundation {
        &self.0[suit as usize]
    }
}

impl CardCollection for Foundations {
    /// Attempts to put a card on the foundation of the appropriate suit.
    ///
    /// An Ace can be put on a foundation iff there is no other card on the foundation of that suit.
    /// This is always the case in a valid game state.
    ///
    /// Any other card *c* can be put on a foundation iff the top card on the foundation of that
    /// suit has a rank exactly one lower than card *c*.
    fn add_card(&self, card: Card) -> Result<Self, ()> {
        // check whether the card can be put on any foundation
        if self.foundation(card.suit).is_empty() {
            // only Aces can be put on an empty foundation
            if card.rank != ACE {
                return Err(());
            }
        } else if
            // Aces can only be put on an empty foundation
            card.rank == ACE ||
            // Other cards can only be put on a foundation if it is one rank higher than the
            // currently topmost card on the foundation
            self.foundation(card.suit).last().unwrap().rank + 1 != card.rank
        {
            return Err(());
        }

        let mut clone = self.clone();
        clone.0[card.suit as usize].push(card);
        Ok(clone)
    }

    /// Always returns an empty Vec, since cards cannot be removed from foundations.
    fn pop_card(&self) -> Vec<(Self, Card)> {
        Vec::with_capacity(0)
    }
}

impl Display for Foundations {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let foundation_strings: Vec<String> = self.0.iter().map(
            |foundation|
                if foundation.is_empty() {
                    "Empty".to_string()
                } else {
                    format!("Up to {}", foundation.last().unwrap().to_string())
                }
        ).collect();
        write!(f, "Foundations: {}", foundation_strings.join(", "))
    }
}

impl Debug for Foundations {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Foundations:")?;
        for foundation in &self.0 {
            if !foundation.is_empty() {
                write!(f, " {:?}", foundation.last().unwrap())?;
            }
        }
        Ok(())
    }
}

impl FromStr for Foundations {
    type Err = String;

    // TODO [v1] document
    // TODO [v1] test
    fn from_str(string: &str) -> Result<Foundations, Self::Err> {
        lazy_static! {
            static ref FOUNDATIONS_RE: Regex = Regex::new(format!(r"(?i)^\s*({}\s*){}$", CARD_PATTERN, "{0,4}").as_str()).unwrap();
            static ref CARD_RE: Regex = Regex::new(format!(r"(?i){}", CARD_PATTERN).as_str()).unwrap();
        }

        if !FOUNDATIONS_RE.is_match(string) {
            return Err(format!("Could not parse foundations: \"{}\"", string))
        }

        let top_cards: Vec<Card> = CARD_RE.find_iter(string).map(|re_match| re_match.as_str().parse().unwrap()).collect();

        let mut foundations = Foundations::new();
        for card in top_cards {
            if !foundations.foundation(card.suit).is_empty() {
                return Err(format!("Multiple foundations of suit {} specified", card.suit));
            } else {
                foundations.0[card.suit as usize] = card_sequence_up_to(card);
            }
        }

        Ok(foundations)
    }
}

fn card_sequence_up_to(card: Card) -> Vec<Card> {
    let mut cards = Vec::new();

    for rank in 1..=card.rank {
        cards.push(Card { suit: card.suit, rank });
    }

    cards
}
