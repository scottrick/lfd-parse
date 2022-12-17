use core::fmt::Debug;
use std::{fmt::Formatter, io::Read};

pub struct ShadingSet {
    pub unknown: [u8; 6],
}

impl ShadingSet {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let mut unknown: [u8; 6] = [0; 6];
        reader
            .read_exact(&mut unknown)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Self { unknown })
    }
}

impl Debug for ShadingSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("ShadingSet{:02X?}", self.unknown);
        f.write_str(&debug_string)
    }
}
