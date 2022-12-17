pub mod mesh_settings;
pub mod mesh_type;
pub mod shading_set;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::io::Read;

use self::mesh_settings::MeshSettings;
use self::shading_set::ShadingSet;

pub struct Ship {
    pub header: LfdHeader,
    pub length: u16,
    pub unknown_1: Vec<u8>,
    pub num_components: u8,
    pub num_shading_sets: u8,
    pub unknown_2: u16,
    pub shading_sets: Vec<ShadingSet>,
    pub mesh_settings: Vec<MeshSettings>,
}

impl LfdResource for Ship {
    fn from_reader(reader: &mut dyn Read, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let length: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading length: {e}"))?;

        let mut unknown_1: Vec<u8> = vec![0; 30];
        reader
            .read_exact(&mut unknown_1)
            .map_err(|e| format!("Error reading ship unknown_1: {e}"))?;

        let num_components: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_components: {e}"))?;

        let num_shading_sets: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading num_shading_sets: {e}"))?;

        let unknown_2: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading unknown_2: {e}"))?;

        let mut shading_sets: Vec<ShadingSet> = Vec::new();
        for _ in 0..num_shading_sets {
            let shading_set = ShadingSet::from_reader(reader)
                .map_err(|e| format!("Error reading shading set: {e}"))?;
            shading_sets.push(shading_set);
        }

        let mut mesh_settings: Vec<MeshSettings> = Vec::new();
        for _ in 0..num_components {
            let mesh_setting = MeshSettings::from_reader(reader)
                .map_err(|e| format!("Error reading mesh setting: {e}"))?;
            mesh_settings.push(mesh_setting);
        }

        // calculate remaining bytes...
        let shading_set_size = usize::from(num_shading_sets) * 6;
        let mesh_settings_size = usize::from(num_components) * 0x40;

        // Read remaining bytes...
        let remaining_bytes: usize =
            usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let remaining_bytes = remaining_bytes - 36 - shading_set_size - mesh_settings_size;

        let mut data: Vec<u8> = vec![0; remaining_bytes];
        reader
            .read_exact(&mut data)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Ship {
            header,
            length,
            unknown_1,
            num_components,
            num_shading_sets,
            unknown_2,
            shading_sets,
            mesh_settings,
        })
    }

    fn to_writer(&self, _writer: &mut dyn std::io::Write) -> Result<(), String> {
        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!(
            "Ship[{} mesh_settings: {:?}]",
            self.header.header_name, self.mesh_settings
        )
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());
    }
}

impl Debug for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
