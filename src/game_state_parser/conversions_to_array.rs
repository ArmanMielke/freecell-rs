use super::error_messages::ERR_TOO_MANY_CASCADES;
use crate::freecell::{Cascade, Cascades};

pub fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<Cascades, String> {
    if cascades.len() > 8 {
        return Err(String::from(ERR_TOO_MANY_CASCADES));
    }

    // pad to 8 cascades
    for _ in cascades.len()..8 {
        cascades.push(Vec::new())
    }

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
