mod lfd_file;
mod lfd_header;

use std::fs;
use std::string::String;

use crate::lfd_file::LfdFile;

fn main() -> Result<(), String> {
    println!("Tie Fighter LFD Tool");

    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        if entry.path().is_file() {
            let _lfd_file = LfdFile::from_file_name(entry.path().to_str().expect("asdf"));
        }
    }

    Ok(())
}
