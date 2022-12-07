use std::fmt::Debug;
use std::fmt::Formatter;
use std::string::String;

use crate::lfd::resources;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_resource::LfdResource;

use std::io::Read;
use std::io::Seek;

pub struct LfdArchive {
    pub resource: Box<dyn LfdResource>,
}

impl LfdArchive {
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, String>
    where
        Self: Sized,
    {
        let resource = resources::create_from_reader(reader)?;
        Ok(LfdArchive { resource })
    }
}

impl Debug for LfdArchive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}

impl LfdPrint for LfdArchive {
    fn lfd_get_print_str(&self) -> String {
        String::from("LfdArchive")
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());

        self.resource.lfd_print(indent + INDENT_SIZE);
    }
}
