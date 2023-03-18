pub mod lfd;

use std::fs;
use std::string::String;

use crate::lfd::lfd_file::LfdFile;
use crate::lfd::traits::lfd_print::LfdPrint;

fn main() -> Result<(), String> {
    println!("LFD Parse Tool");

    let _create_dir_result = fs::create_dir("out/");

    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        let is_species = entry.path().starts_with("data/SPECIES.LFD");
        // || entry.path().starts_with("data/SPECIES2.LFD")
        // || entry.path().starts_with("data/SPECIES3.LFD");

        if entry.path().is_file() && is_species {
            let lfd_file =
                LfdFile::read_from_file(entry.path().to_str().expect("Failed to get file name."))?;

            lfd_file.lfd_print(0);

            // let output_file = LfdFile {
            //     file_name: lfd_file.file_name.replace("data/", "out/"),
            //     archive: lfd_file.archive,
            // };
            // output_file.write_to_file()?;
        }
    }

    Ok(())
}
