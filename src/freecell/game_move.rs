use super::Card;
use super::position::Position;


pub struct Move {
    pub card: Card,
    pub from: Position,
    pub to: Position,
}
