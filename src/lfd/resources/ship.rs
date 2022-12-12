use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::io::Read;

pub struct Ship {
    pub header: LfdHeader,
}

impl LfdResource for Ship {
    fn from_reader(reader: &mut dyn Read, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let size_left: usize =
            usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let size_left = size_left;
        let mut data: Vec<u8> = vec![0; size_left];

        reader
            .read_exact(&mut data)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Ship { header })
    }

    fn to_writer(&self, _writer: &mut dyn std::io::Write) -> Result<(), String> {
        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!("Ship[{}]", self.header.header_name,)
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}

impl Debug for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
