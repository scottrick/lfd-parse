use std::{
    fmt::{Debug, Formatter},
    io::Read,
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::lfd::traits::lfd_print::LfdPrint;

pub struct LodHeader {
    pub distance: i32,
    pub offset: u16,
}

impl LodHeader {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let distance: i32 = reader
            .read_i32::<LittleEndian>()
            .map_err(|e| format!("Error reading distance: {e}"))?;

        let offset: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading offset: {e}"))?;

        Ok(Self { distance, offset })
    }
}

impl Debug for LodHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "LodHeader distance: {:?} offset: {:?}",
            self.distance, self.offset
        );
        f.write_str(&debug_string)
    }
}

impl LfdPrint for LodHeader {
    fn lfd_print(&self, indent: usize) {
        // let spaces = " ".repeat(indent);
        // println!("{spaces}{}", self.lfd_get_print_str());
    }

    fn lfd_get_print_str(&self) -> String {
        format!("{:?}", self)
    }
}
