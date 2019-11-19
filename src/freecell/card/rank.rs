
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;
pub const ACE: Rank = 1;

/// Indicates the rank of a card.
///
/// The ranks are as follows:
/// - An Ace has rank 1
/// - Numbered cards have the rank corresponding to their numbers
/// - A Jack has rank 11
/// - A Queen has rank 12
/// - A King has rank 13
///
/// Rank 0 and ranks greater than 13 are not valid card ranks.
pub type Rank = u8;
