use std::fs;

use lfd::lfd::lfd_file::LfdFile;

#[test]
pub fn compare_all_lfd() -> Result<(), String> {
    let _create_dir_result = fs::create_dir("test_dir/");

    // For every LFD file in the data/ directory, run read_write_comparison.
    for entry in fs::read_dir("data/").map_err(|e| format!("Error reading directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;

        let is_lfd = entry
            .path()
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("lfd"));

        let _is_battle = entry
            .path()
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case("battle3"));

        match entry.path().is_file() && is_lfd {
            true => {
                let mut lfd_file = LfdFile::read_from_file(
                    entry.path().to_str().expect("Failed to get file name."),
                )?;

                read_write_comparison(&mut lfd_file);
            }
            false => (),
        }
    }

    let _remove_dir_result = fs::remove_dir_all("test_dir/");

    Ok(())
}

fn read_write_comparison(lfd_file: &mut LfdFile) {
    println!("read_write_comparison: {}", lfd_file.file_name);

    // Open original LFD file.
    let original = std::fs::read(&lfd_file.file_name).expect("Failed to open original LFD file.");

    // let test_dir = "test_dir/";
    // Expand test LFD to test dir.
    // lfd_file
    //     .archive
    //     .expand_to_destination(test_dir)
    //     .expect("Failed to expand.");

    // Write out in-memory LFD data for comparison.
    lfd_file.file_name = String::from("test_dir/TEST.LFD");
    lfd_file
        .write_to_file()
        .expect("Failed to write duplicate LFD file.");

    // Open duplicate LFD file.
    let duplicate = std::fs::read(&lfd_file.file_name).expect("Failed to open duplicate LFD file.");

    // Binary comparison between original and in_memory_extract version.
    assert_eq!(original.len(), duplicate.len());

    // Compare every byte next.
    for i in 0..original.len() {
        assert_eq!(original[i], duplicate[i]);
    }

    fs::remove_file(&lfd_file.file_name).expect("Failed to clean up duplicate file.");
}
