use crate::lfd::traits::lfd_print::LfdPrint;

use core::fmt::Debug;

use std::{fmt::Formatter, fs::File, io::BufReader};

use super::vertex16::Vertex16;

pub struct MeshVertices {
    vertices: Vec<Vertex16>,
}

impl MeshVertices {
    pub fn from_reader(reader: &mut BufReader<File>, num_vertices: u8) -> Result<Self, String> {
        let mut vertices: Vec<Vertex16> = Vec::new();

        // println!("=====================================");
        // println!("Reading {:?} vertices.", num_vertices);

        for i in 0..num_vertices {
            let mut new_vertex =
                Vertex16::from_reader(reader).map_err(|e| format!("Error reading vertex: {e}"))?;

            // println!("  {new_vertex:?}");

            let x = new_vertex.x >> 8;
            let y = new_vertex.y >> 8;
            let z = new_vertex.z >> 8;
            let mut _something_was_changed = false;

            if x == 0x007f {
                let right_byte = 0x00ff & new_vertex.x;
                let shifted = (right_byte >> 1) as u8;
                let index_to_use = i - shifted;
                let x_to_use = vertices[index_to_use as usize].x;
                new_vertex.x = x_to_use;
                _something_was_changed = true;
            }

            if y == 0x007f {
                let right_byte = 0x00ff & new_vertex.y;
                let shifted = (right_byte >> 1) as u8;
                let index_to_use = i - shifted;
                let y_to_use = vertices[index_to_use as usize].y;
                new_vertex.y = y_to_use;
                _something_was_changed = true;
            }

            if z == 0x007f {
                let right_byte = 0x00ff & new_vertex.z;
                let shifted = (right_byte >> 1) as u8;
                let index_to_use = i - shifted;
                let z_to_use = vertices[index_to_use as usize].z;
                new_vertex.z = z_to_use;
                _something_was_changed = true;
            }

            // if _something_was_changed {
            //     println!("X {new_vertex:?}");
            // }

            vertices.push(new_vertex);
        }

        Ok(MeshVertices { vertices })
    }

    pub fn obj_print(&self) {
        for vertex in self.vertices.iter() {
            vertex.obj_print();
        }
    }
}

impl Debug for MeshVertices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("MeshVertices[{:?}]", self.vertices.len());
        f.write_str(&debug_string)
    }
}

impl LfdPrint for MeshVertices {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
