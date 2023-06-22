use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Pltt {
    pub header: LfdHeader,
    pub start_index: u8,
    pub end_index: u8,
    // pub left: u16,
    // pub top: u16,
    // pub right: u16,
    // pub bottom: u16,
}

impl LfdResource for Pltt {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let start_index: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading start_index: {e}"))?;

        let end_index: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading end_index: {e}"))?;

        // let bottom: u16 = reader
        //     .read_u16::<LittleEndian>()
        //     .map_err(|e| format!("Error reading bottom: {e}"))?;

        let size_left: usize =
            usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let size_left = size_left - 2;
        let mut data: Vec<u8> = vec![0; size_left];

        reader
            .read_exact(&mut data)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Pltt {
            header,
            start_index,
            end_index,
        })
    }

    fn to_writer(&self, _writer: &mut dyn std::io::Write) -> Result<(), String> {
        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!(
            "Pltt[{}], [{:?} {:?}]",
            self.header.header_name, self.start_index, self.end_index
        )
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}

impl Debug for Pltt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
