use byteorder::{LittleEndian, ReadBytesExt};

use crate::lfd::traits::lfd_print::LfdPrint;

use core::fmt::Debug;

use std::{fmt::Formatter, fs::File, io::BufReader};

pub struct Vertex16 {
    x: i16,
    y: i16,
    z: i16,
}

impl Vertex16 {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let x: i16 = reader
            .read_i16::<LittleEndian>()
            .map_err(|e| format!("Error reading x: {e}"))?;
        let y: i16 = reader
            .read_i16::<LittleEndian>()
            .map_err(|e| format!("Error reading y: {e}"))?;
        let z: i16 = reader
            .read_i16::<LittleEndian>()
            .map_err(|e| format!("Error reading z: {e}"))?;

        Ok(Vertex16 { x, y, z })
    }
}

impl Debug for Vertex16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("Vertex16 ({:?}, {:?}, {:?})", self.x, self.y, self.z,);
        f.write_str(&debug_string)
    }
}

impl LfdPrint for Vertex16 {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
