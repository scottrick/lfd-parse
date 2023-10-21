pub mod lod_header;
pub mod lod_mesh;
pub mod mesh_settings;
pub mod mesh_type;
pub mod shading_set;
pub mod shape;
pub mod shape_settings;
pub mod ship_component;
pub mod unknown1;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use self::mesh_settings::MeshSettings;
use self::shading_set::ShadingSet;
use self::ship_component::ShipComponent;

pub struct Ship {
    pub header: LfdHeader,
    pub length: u16,
    pub unknown_1: Vec<u8>,
    pub num_components: u8,
    pub num_shading_sets: u8,
    pub unknown_2: u16,
    pub shading_sets: Vec<ShadingSet>,
    pub mesh_settings: Vec<MeshSettings>,
    pub ship_components: Vec<ShipComponent>,
}

impl LfdResource for Ship {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let length: u16 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| format!("Error reading length: {e}"))?;

        let mesh_cursor_pos = reader
            .stream_position()
            .map_err(|e| format!("Error reading stream position: {e}"))?;
        let end_pos = mesh_cursor_pos + length as u64;

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

        let mut ship_components: Vec<ShipComponent> = Vec::new();
        for settings in mesh_settings.iter() {
            // settings.component_offset
            reader
                .seek(SeekFrom::Start(settings.component_start))
                .map_err(|e| format!("Error seeing ship component: {e}"))?;

            let ship_component = ShipComponent::from_reader(reader)
                .map_err(|e| format!("Error reading ship component: {e}"))?;

            ship_components.push(ship_component);
        }

        // Seek to the end of this ship component.
        reader
            .seek(SeekFrom::Start(end_pos))
            .map_err(|e| format!("Error seeking end of ship: {e}"))?;

        Ok(Ship {
            header,
            length,
            unknown_1,
            num_components,
            num_shading_sets,
            unknown_2,
            shading_sets,
            mesh_settings,
            ship_components,
        })
    }

    fn to_writer(&self, _writer: &mut dyn std::io::Write) -> Result<(), String> {
        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!("Ship[{}]", self.header.header_name)
    }

    fn lfd_print(&self, indent: usize) {
        let settings_size = self.mesh_settings.len();
        let components_size = self.ship_components.len();

        if settings_size != components_size {
            panic!("Invalid length.");
        }

        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());

        println!("  {spaces}ShadingSets[{}]", self.shading_sets.len());
        for shading_set in self.shading_sets.iter() {
            println!("    {spaces}{shading_set:?}");
        }

        for i in 0..settings_size {
            let setting = &self.mesh_settings[i];
            let component = &self.ship_components[i];

            setting.lfd_print(indent + 2);
            component.lfd_print(indent + 2);
        }
    }
}

impl Debug for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}

impl Ship {
    pub fn write_to_obj_file(&self) -> Result<(), String> {
        let _create_dir_result = fs::create_dir("obj/");
        let file = File::create(format!("obj/{}.obj", self.header.header_name.clone()))
            .map_err(|e| format!("Error writing obj file: {e}"))?;

        let mut writer = BufWriter::new(file);
        let mut next_vertex_index: usize = 0;

        for ship_component in self.ship_components.iter() {
            ship_component.obj_to_writer(&mut writer, &mut next_vertex_index)?;
        }

        println!(
            "Finished writing {} vertices for {}.",
            next_vertex_index, self.header.header_name,
        );

        Ok(())
    }
}
