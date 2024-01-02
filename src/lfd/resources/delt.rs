use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Delt {
    pub header: LfdHeader,
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub remainder: Vec<u8>,
}

impl LfdResource for Delt {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let left: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading left: {e}"))?;

        let top: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading top: {e}"))?;

        let right: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading right: {e}"))?;

        let bottom: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading bottom: {e}"))?;

        let size_left: usize =
            usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let size_left = size_left - 8;
        let mut remainder: Vec<u8> = vec![0; size_left];

        reader
            .read_exact(&mut remainder)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Delt {
            header,
            left,
            top,
            right,
            bottom,
            remainder,
        })
    }

    fn to_writer(&self, writer: &mut dyn std::io::Write) -> Result<(), String> {
        self.header.to_writer(writer)?;

        writer
            .write_u16::<LittleEndian>(self.left)
            .map_err(|e| format!("Error writing left: {e}"))?;
        writer
            .write_u16::<LittleEndian>(self.top)
            .map_err(|e| format!("Error writing top: {e}"))?;
        writer
            .write_u16::<LittleEndian>(self.right)
            .map_err(|e| format!("Error writing right: {e}"))?;
        writer
            .write_u16::<LittleEndian>(self.bottom)
            .map_err(|e| format!("Error writing bottom: {e}"))?;

        writer
            .write_all(&self.remainder)
            .map_err(|e| format!("Error writing remainder: {e}"))?;

        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!(
            "Delt[{}] left[{:?}] top[{:?}] right[{:?}] bottom[{:?}]",
            self.header.header_name, self.left, self.top, self.right, self.bottom,
        )
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}

impl Debug for Delt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
