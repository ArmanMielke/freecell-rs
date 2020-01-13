use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::GameState;

#[allow(missing_docs)]
pub fn parse_file<P: AsRef<Path>>(file_name: P) -> Result<GameState, String> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => return Err("File could not be read".to_string()),
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return Err("File contents could not be read".to_string())
    }

    contents.parse()
}
