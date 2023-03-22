use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::{BufReader, Read},
};

use byteorder::ReadBytesExt;

use crate::lfd::traits::lfd_print::LfdPrint;

pub struct Shape {
    pub combo_byte: u8,
    pub shape_type: u8,
    pub num_vertices: u8,
    pub shape_data: Vec<u8>,
}

impl Shape {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let combo_byte: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading shape_type: {e}"))?;

        let shape_type = combo_byte >> 4;
        let num_vertices = combo_byte & 0b00001111;

        let mut shape_data: Vec<u8> = vec![0; num_vertices as usize];

        reader
            .read_exact(&mut shape_data)
            .map_err(|e| format!("Error reading shape_data: {e}"))?;

        Ok(Shape {
            combo_byte,
            shape_type,
            num_vertices,
            shape_data,
        })
    }
}

impl Debug for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "Shape combo: {:08b}, shape_type: {:?}, num_vertices: {:?}, data: {:?}",
            self.combo_byte, self.shape_type, self.num_vertices, self.shape_data,
        );

        f.write_str(&debug_string)
    }
}

impl LfdPrint for Shape {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
