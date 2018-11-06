use super::card::Card;

const FOUNDATION_MAX_SIZE: usize = 13;

/// May only contain cards of one suit, ordered from Ace upwards (this is not enforced).
pub type Foundation = Vec<Card>;
