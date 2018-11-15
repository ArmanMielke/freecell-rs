use super::Card;



//const CASCADE_MAX_SIZE: usize = 52;


/// A stack of arbitrary cards.
/// May only be used as a queue (TODO: enforce this).
pub type Cascade = Vec<Card>;

pub type Cascades = [Cascade; 8];
