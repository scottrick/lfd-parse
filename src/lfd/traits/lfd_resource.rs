use crate::lfd::lfd_header::LfdHeader;

use std::io::Read;
use std::io::Write;

pub trait LfdResource {
    fn from_reader(reader: &mut dyn Read, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized;

    fn to_writer(&self, writer: &mut dyn Write) -> Result<(), String>;

    fn get_lfd_header(&self) -> &LfdHeader;

    fn lfd_get_print_str(&self) -> String;

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}
