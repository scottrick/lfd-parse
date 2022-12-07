use crate::lfd::lfd_header::LfdHeader;

use std::io::Read;
use std::io::Seek;

pub trait LfdResource {
    fn get_lfd_header(&self) -> &LfdHeader;

    fn from_reader(reader: &mut (impl Read + Seek), header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized;

    fn lfd_get_print_str(&self) -> String;

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}
