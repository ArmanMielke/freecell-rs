use super::Card;
use super::position::Position;


pub struct GameMove {
    pub card: Card,
    pub from: Position,
    pub to: Position,
}
