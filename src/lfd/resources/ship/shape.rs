use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::{BufReader, Read},
};

use byteorder::ReadBytesExt;

use crate::lfd::traits::lfd_print::LfdPrint;

pub struct Shape {
    pub first_byte: u8,
    pub shade_flag: bool,
    pub two_sided_flag: bool,
    pub num_vertices: u8,
    pub shape_data: Vec<u8>,
}

impl Shape {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let first_byte: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading shape_type: {e}"))?;

        // top nibble contains two flags
        let shade_flag = first_byte & 0x40 != 0;
        let two_sided_flag = first_byte & 0x80 != 0;

        // bottom nibble is the number of vertices
        let num_vertices = first_byte & 0x0f;

        let mut shape_data: Vec<u8> = vec![0; 3 + (num_vertices * 2) as usize];

        reader
            .read_exact(&mut shape_data)
            .map_err(|e| format!("Error reading shape_data: {e}"))?;

        Ok(Shape {
            first_byte,
            shade_flag,
            two_sided_flag,
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
                //+1 because OBJ format indices start at 1
                self.shape_data[2] as usize + vertex_offset + 1,
                //+1 because OBJ format indices start at 1
                self.shape_data[3] as usize + vertex_offset + 1
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
                    //+1 because OBJ format indices start at 1
                    i1 + vertex_offset + 1,
                    //+1 because OBJ format indices start at 1
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
            "Shape first_byte: {:08b}, shade_flag: {}, two_sided_flag: {}, num_vertices: {}, data: {:?}",
            self.first_byte, self.shade_flag, self.two_sided_flag, self.num_vertices, self.shape_data,
        );

        f.write_str(&debug_string)
    }
}

impl LfdPrint for Shape {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
