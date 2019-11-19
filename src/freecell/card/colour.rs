use std::fmt::{Display, Formatter, Result};



#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Colour {
    Black, Red
}


impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Colour::Black => write!(f, "black"),
            Colour::Red => write!(f, "red"),
        }
    }
}
