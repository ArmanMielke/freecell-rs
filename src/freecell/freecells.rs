use super::Card;

use arrayvec::ArrayVec;



/// May hold up to four arbitrary cards
pub type Freecells = ArrayVec<[Card; 4]>;
