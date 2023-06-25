use byteorder::ReadBytesExt;

use crate::lfd::def::color_array::ColorArray;
use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;

pub struct Pltt {
    pub header: LfdHeader,
    pub start_index: u8,
    pub end_index: u8,
    pub colors: ColorArray,
    pub reserved: u8,
}

impl LfdResource for Pltt {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let start_index: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading start_index: {e}"))?;

        let end_index: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading end_index: {e}"))?;

        let num_colors = end_index - start_index + 1;
        let colors = ColorArray::from_reader(reader, num_colors)
            .map_err(|e| format!("Error reading color array: {e}"))?;

        let reserved: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading reserved byte: {e}"))?;

        Ok(Pltt {
            header,
            start_index,
            end_index,
            colors,
            reserved,
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
            "Pltt[{}], [{:?} {:?}]",
            self.header.header_name, self.start_index, self.end_index
        )
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());

        // Print out the colors in the palette.
        for color in self.colors.colors.iter() {
            println!("{}", color.get_8bit_color_str());
            // color.lfd_print(indent + 2)
        }
    }
}

impl Debug for Pltt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
