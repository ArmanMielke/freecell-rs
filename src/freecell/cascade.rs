use super::Card;

const CASCADE_MAX_SIZE: usize = 52;

// TODO use type alias instead?
#[derive(Debug, PartialEq)]
pub struct Cascade (pub Vec<Card>);
