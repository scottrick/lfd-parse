use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::string::String;

use crate::lfd::lfd_archive::LfdArchive;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_reader::LfdReader;

pub struct LfdFile {
    pub file_name: String,
    pub archive: LfdArchive,
}

impl LfdFile {
    pub fn read_from_file(file_name: &str) -> Result<Self, String> {
        let file =
            File::open(file_name).map_err(|e| format!("Unable to open file {file_name}: {e}"))?;

        let mut reader = BufReader::new(file);

        let archive = LfdArchive::from_reader(&mut reader)?;

        Ok(LfdFile {
            file_name: file_name.to_string(),
            archive,
        })
    }
}

impl Debug for LfdFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LfdFile")?;
        write!(f, "  {:?}", self.archive)?;

        Ok(())
    }
}

impl LfdPrint for LfdFile {
    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}LfdFile [{}]", self.file_name);
        self.archive.lfd_print(indent + INDENT_SIZE);
    }
}
