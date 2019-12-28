use super::error_messages::ERR_TOO_MANY_CASCADES;
use crate::{Cascade, Cascades};

pub fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<Cascades, String> {
    // make sure cascades has length 8
    if cascades.len() > 8 {
        return Err(String::from(ERR_TOO_MANY_CASCADES));
    }
    cascades.resize(8, Vec::new());

    // convert to array
    Ok([
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
        cascades.remove(0),
    ])
}
