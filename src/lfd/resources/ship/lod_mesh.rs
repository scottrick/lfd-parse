use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use byteorder::ReadBytesExt;

use crate::lfd::traits::lfd_print::LfdPrint;

use super::vertex16::Vertex16;

// LodMesh
/// {
///   /* 0x00 */ byte Signature
///   /* 0x01 */ byte Unknown
///   /* 0x02 */ byte NumVertices
///   /* 0x03 */ byte Unknown
///   /* 0x04 */ byte NumShapes
///   /* 0x05 */ byte[NumShapes] ColorIndices
///              Vertex16 MinimumBound
///              Vertex16 MaximumBound
///              Vertex16[NumVertices] MeshVertices
///              Vertex16[NumVertices] VertexNormals
///              ShapeSettings[NumShapes]
///              Shape[NumShapes]    MeshGeometry
///              Unknown1[NumShapes]
///              short NumUnk2
///              Unknown2[NumUnk2]
///              Unknown3[NumUnk2]
/// }
pub struct LodMesh {
    signature: u8,
    unknown_1: u8,
    num_vertices: u8,
    unknown_2: u8,
    num_shapes: u8,
    color_indices: Vec<u8>,
    min_bound: Vertex16,
    max_bound: Vertex16,
}

impl LodMesh {
    pub fn from_reader(reader: &mut BufReader<File>) -> Result<Self, String> {
        let signature: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading signature: {e}"))?;

        let unknown_1: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading unknown_1: {e}"))?;

        let num_vertices: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_vertices: {e}"))?;

        let unknown_2: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading unknown_2: {e}"))?;

        let num_shapes: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_shapes: {e}"))?;

        let mut color_indices: Vec<u8> = vec![0; num_shapes as usize];

        reader
            .read_exact(&mut color_indices)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        let min_bound =
            Vertex16::from_reader(reader).map_err(|e| format!("Error reading Vertex16: {e}"))?;

        let max_bound =
            Vertex16::from_reader(reader).map_err(|e| format!("Error reading Vertex16: {e}"))?;

        Ok(Self {
            signature,
            unknown_1,
            num_vertices,
            unknown_2,
            num_shapes,
            color_indices,
            min_bound,
            max_bound,
        })
    }
}

impl Debug for LodMesh {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "LodMesh signature: {:?} u1: {:?} num_vertices: {:?} u2: {:?} num_shapes: {:?} color_indices: {:?}, min_bound: {:?}, max_bound: {:?}",
            self.signature, self.unknown_1, self.num_vertices, self.unknown_2, self.num_shapes, self.color_indices, self.min_bound, self.max_bound
        );
        f.write_str(&debug_string)
    }
}

impl LfdPrint for LodMesh {
    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }

    fn lfd_get_print_str(&self) -> String {
        format!("{:?}", self)
    }
}
