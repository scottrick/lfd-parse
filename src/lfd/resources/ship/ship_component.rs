use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, Seek};

use crate::lfd::traits::lfd_print::LfdPrint;

use super::lod_header::LodHeader;
use super::lod_mesh::LodMesh;

pub struct ShipComponent {
    pub lod_headers: Vec<LodHeader>,
    pub lod_meshes: Vec<LodMesh>,
}

impl ShipComponent {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let mut lod_headers: Vec<LodHeader> = Vec::new();
        let mut lod_meshes: Vec<LodMesh> = Vec::new();

        let mut has_more_headers = true;

        while has_more_headers {
            let header_cursor_pos = reader
                .stream_position()
                .map_err(|e| format!("Error reading stream position: {e}"))?;

            let header = LodHeader::from_reader(reader)
                .map_err(|e| format!("Error reading LodHeader: {e}"))?;

            if header.distance == 0x7fffffff {
                has_more_headers = false;
            }

            reader
                .seek(std::io::SeekFrom::Start(
                    header_cursor_pos + header.offset as u64,
                ))
                .map_err(|e| format!("Unable to seek to lod mesh: {e}"))?;

            let mesh =
                LodMesh::from_reader(reader).map_err(|e| format!("Error reading LodMesh: {e}"))?;

            reader
                .seek(std::io::SeekFrom::Start(header_cursor_pos + 6))
                .map_err(|e| format!("Unable to seek to next lod header: {e}"))?;

            lod_headers.push(header);
            lod_meshes.push(mesh);
        }

        Ok(Self {
            lod_headers,
            lod_meshes,
        })
    }
}

impl Debug for ShipComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = "ShipComponent".to_string();
        f.write_str(&debug_string)
    }
}

impl LfdPrint for ShipComponent {
    fn lfd_print(&self, indent: usize) {
        let headers_size = self.lod_headers.len();
        let meshes_size = self.lod_meshes.len();

        if headers_size != meshes_size {
            panic!("Invalid length.");
        }

        for i in 0..headers_size {
            let header = &self.lod_headers[i];
            let mesh = &self.lod_meshes[i];

            header.lfd_print(indent + 2);
            mesh.lfd_print(indent + 2);
        }
    }

    fn lfd_get_print_str(&self) -> String {
        format!("{:?}", self)
    }
}
