use std::{
    fmt::{Debug, Formatter},
    io::Read,
};

use byteorder::{LittleEndian, ReadBytesExt};

use super::mesh_type::MeshType;

pub struct MeshSettings {
    pub mesh_type: MeshType,
    pub unknown_1: [u8; 0x2a],
    pub component_offset: u16,
    pub unknown_2: [u8; 0x12],
}

impl MeshSettings {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let mesh_type: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading length: {e}"))?;
        let mesh_type = MeshType::from(mesh_type);

        let mut unknown_1: [u8; 0x2a] = [0; 0x2a];
        reader
            .read_exact(&mut unknown_1)
            .map_err(|e| format!("Error reading unknown_1 buffer: {e}"))?;

        let component_offset: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading length: {e}"))?;

        let mut unknown_2: [u8; 0x12] = [0; 0x12];
        reader
            .read_exact(&mut unknown_2)
            .map_err(|e| format!("Error reading unknown_2 buffer: {e}"))?;

        Ok(Self {
            mesh_type,
            unknown_1,
            component_offset,
            unknown_2,
        })
    }
}

impl Debug for MeshSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("{:?}", self.mesh_type,);
        f.write_str(&debug_string)
    }
}
