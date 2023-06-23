use crate::lfd::traits::lfd_print::LfdPrint;

use core::fmt::Debug;

use std::{fmt::Formatter, fs::File, io::BufReader};

use super::vertex16::Vertex16;

pub struct VertexArray {
    pub vertices: Vec<Vertex16>,
}

impl VertexArray {
    ///
    /// repeat_check will check if the top byte is 0x7f.  
    /// If so, that vertex is a repeat from a previous vertex.
    ///
    pub fn from_reader(
        reader: &mut BufReader<File>,
        num_vertices: u8,
        repeat_check: bool,
    ) -> Result<Self, String> {
        let mut vertices: Vec<Vertex16> = Vec::new();

        for i in 0..num_vertices {
            let mut new_vertex =
                Vertex16::from_reader(reader).map_err(|e| format!("Error reading vertex: {e}"))?;

            if repeat_check {
                let x = new_vertex.x >> 8;
                let y = new_vertex.y >> 8;
                let z = new_vertex.z >> 8;

                if x == 0x007f {
                    let right_byte = 0x00ff & new_vertex.x;
                    let shifted = (right_byte >> 1) as u8;
                    let index_to_use = i - shifted;
                    let x_to_use = vertices[index_to_use as usize].x;
                    new_vertex.x = x_to_use;
                }

                if y == 0x007f {
                    let right_byte = 0x00ff & new_vertex.y;
                    let shifted = (right_byte >> 1) as u8;
                    let index_to_use = i - shifted;
                    let y_to_use = vertices[index_to_use as usize].y;
                    new_vertex.y = y_to_use;
                }

                if z == 0x007f {
                    let right_byte = 0x00ff & new_vertex.z;
                    let shifted = (right_byte >> 1) as u8;
                    let index_to_use = i - shifted;
                    let z_to_use = vertices[index_to_use as usize].z;
                    new_vertex.z = z_to_use;
                }
            }

            vertices.push(new_vertex);
        }

        Ok(VertexArray { vertices })
    }
}

impl Debug for VertexArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("VertexArray [{:?}]", self.vertices.len());
        f.write_str(&debug_string)
    }
}

impl LfdPrint for VertexArray {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
