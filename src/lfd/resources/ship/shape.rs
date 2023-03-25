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

        // top nibble is the type
        let shape_type = combo_byte >> 4;
        // bottom nibble is the number of vertices
        let num_vertices = combo_byte & 0b00001111;

        let mut shape_data: Vec<u8> = vec![0; 3 + (num_vertices * 2) as usize];

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

    pub fn obj_to_writer(
        &self,
        writer: &mut dyn std::io::Write,
        vertex_offset: usize,
    ) -> Result<(), String> {
        if self.num_vertices == 2 {
            // single line
            writeln!(
                writer,
                "l {:?} {:?}",
                self.shape_data[2] as usize + vertex_offset,
                self.shape_data[3] as usize + vertex_offset
            )
            .map_err(|e| format!("Error writing shape to obj writer: {e}"))?;
        } else {
            // Data[v * 2] and Data[(v + 1) * 2]
            for i in 0..self.num_vertices as usize {
                let i1 = self.shape_data[i * 2] as usize;
                let i2 = self.shape_data[(i + 1) * 2] as usize;

                writeln!(
                    writer,
                    "l {:?} {:?}",
                    i1 + vertex_offset + 1,
                    i2 + vertex_offset + 1
                )
                .map_err(|e| format!("Error writing shape to obj writer: {e}"))?;
            }
        }

        Ok(())
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
