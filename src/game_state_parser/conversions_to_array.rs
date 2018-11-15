use freecell::{Cascade, Cascades};
use super::error_messages::ERR_TOO_MANY_CASCADES;


pub fn cascades_vec_to_array(mut cascades: Vec<Cascade>) -> Result<Cascades, String> {
    // TODO implement this properly
    if cascades.len() == 8 {
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
    } else if cascades.len() > 8 {
        Err(String::from(ERR_TOO_MANY_CASCADES))
    } else if cascades.len() == 7 {
        Ok([
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            cascades.remove(0),
            Vec::new(),
        ])
    } else {
        unimplemented!()
    }
}
