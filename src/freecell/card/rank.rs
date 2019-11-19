
pub const ACE: Rank = 1;
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
pub const KING: Rank = 13;


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


pub fn rank_from_string(string: String) -> Result<Rank, &'static str> {
    match string.trim().to_lowercase().as_ref() {
        "ace" => Ok(ACE),
        "a" => Ok(ACE),
        "jack" => Ok(JACK),
        "j" => Ok(JACK),
        "queen" => Ok(QUEEN),
        "q" => Ok(QUEEN),
        "king" => Ok(KING),
        "k" => Ok(KING),

        // rank is not a word => try to parse it as u8
        _ => match string.trim().parse::<u8>() {
            Err(_) => return Err("Could not parse integer"),

            // successfully parsed number => check whether it is in the correct range
            Ok(rank) => match rank {
                0 => Err("Rank cannot be 0"),
                1...13 => Ok(rank),
                _ => Err("Rank cannot be greater than 13"),
            }
        }
    }
}
