use core::fmt::Debug;
use std::{fmt::Formatter, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};

pub struct ShadingSet {
    // pub unknown: [u8; 6],
    pub offset: u16,
    pub unknown: [u8; 4],
}

impl ShadingSet {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let offset: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading offset: {e}"))?;

        let mut unknown: [u8; 4] = [0; 4];
        reader
            .read_exact(&mut unknown)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Self { offset, unknown })
    }
}

impl Debug for ShadingSet {
    // fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //     let debug_string = format!("ShadingSet{:02X?}", self.unknown);
    //     f.write_str(&debug_string)
    // }

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "ShadingSet offset: {:?} unknown: {:02X?}",
            self.offset, self.unknown,
        );
        f.write_str(&debug_string)
    }
}
