use std::fmt::Debug;
use std::fmt::Formatter;
use std::string::String;

use crate::lfd::components::rmap::Rmap;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_reader::LfdReader;

pub struct LfdArchive {
    pub rmap: Rmap, //need to change to handle any type.  cockpit lfd evidently don't start with
                    //rmap.
}

impl LfdReader for LfdArchive {
    fn from_reader(reader: &mut impl std::io::Read) -> Result<Self, String>
    where
        Self: Sized,
    {
        let rmap = Rmap::from_reader(reader)?;
        Ok(LfdArchive { rmap })
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

        self.rmap.lfd_print(indent + INDENT_SIZE);
    }
}
