use std::fs::File;
use std::io::BufReader;
use std::string::String;

use super::lfd_archive::LfdArchive;
use super::traits::lfd_reader::LfdReader;

pub struct LfdFile {
    pub archive: LfdArchive,
}

impl LfdFile {
    pub fn read_from_file(file_name: &str) -> Result<Self, String> {
        let file =
            File::open(file_name).map_err(|e| format!("Unable to open file {file_name}: {e}"))?;

        let mut reader = BufReader::new(file);

        let archive = LfdArchive::from_reader(&mut reader)?;

        /*
        let lfd_header = LfdHeader::read_from_buffer(&mut buf_reader)?;

        println!("{file_name}");
        println!("    {lfd_header:?}");

        let num_sub_headers = lfd_header.size / 0x10;

        for _ in 0..num_sub_headers {
            let sub_header = LfdHeader::read_from_buffer(&mut buf_reader)?;
            println!("        {sub_header:?}");
        }

        Ok(LfdFile {
            file_name: file_name.to_string(),
            lfd_header,
        })
            */

        Ok(LfdFile { archive })
    }
}
