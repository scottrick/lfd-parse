use crate::lfd::lfd_header::LfdHeader;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

pub trait LfdResource {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized;

    fn to_writer(&self, writer: &mut dyn Write) -> Result<(), String>;

    fn get_lfd_header(&self) -> &LfdHeader;

    fn lfd_get_print_str(&self) -> String;

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }

    fn expand_to_destination(&self, destination: &str) -> Result<(), String> {
        let file = self.create_extract_file(destination)?;
        let mut writer = BufWriter::new(file);

        self.to_writer(&mut writer)?;

        Ok(())
    }

    fn create_extract_file(&self, destination: &str) -> Result<File, String> {
        let file_name = format!(
            "{destination}/{}.{:?}",
            self.get_lfd_header().header_name,
            self.get_lfd_header().header_type
        );

        File::create(&file_name)
            .map_err(|e| format!("[{file_name}] Error creating file for extraction: {e}"))
    }
}
