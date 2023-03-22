use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::BufReader,
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::lfd::traits::lfd_print::LfdPrint;

use super::vertex16::Vertex16;

pub struct ShapeSettings {
    pub normal: Vertex16,
    pub offset: i16,
}

impl ShapeSettings {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let normal =
            Vertex16::from_reader(reader).map_err(|e| format!("Error reading Vertex16: {e}"))?;

        let offset: i16 = reader
            .read_i16::<LittleEndian>()
            .map_err(|e| format!("Error reading offset: {e}"))?;

        Ok(ShapeSettings { normal, offset })
    }
}

impl Debug for ShapeSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "ShapeSettings normal: {:?} offset: {:?}",
            self.normal, self.offset,
        );

        f.write_str(&debug_string)
    }
}

impl LfdPrint for ShapeSettings {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
