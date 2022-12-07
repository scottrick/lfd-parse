pub mod lfd;

use std::fs;
use std::string::String;

use crate::lfd::lfd_file::LfdFile;
use crate::lfd::traits::lfd_print::LfdPrint;

fn main() -> Result<(), String> {
    println!("LFD Parse Tool");

    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        if entry.path().is_file() {
            let lfd_file =
                LfdFile::read_from_file(entry.path().to_str().expect("Failed to get file name."))?;

            lfd_file.lfd_print(0);
        }
    }

    Ok(())
}
