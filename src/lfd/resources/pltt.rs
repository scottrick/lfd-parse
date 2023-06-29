use byteorder::ReadBytesExt;

use crate::lfd::def::color_array::ColorArray;
use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_resource::LfdResource;

use core::fmt::Debug;
use core::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Pltt {
    pub header: LfdHeader,
    pub start_index: u8,
    pub end_index: u8,
    pub colors: ColorArray,
    pub reserved: Vec<u8>,
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

        let num_colors = end_index as usize - start_index as usize + 1;
        let colors = ColorArray::from_reader(reader, num_colors)
            .map_err(|e| format!("Error reading color array: {e}"))?;

        let total_size: usize =
            usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let size_read = 2 + num_colors * 3;

        // A few of the palettes have more than 1 byte "reserved" at the end.
        // Not sure why, but CITY.LFD does, for example.
        let size_remaining = total_size - size_read;
        let mut reserved: Vec<u8> = vec![0; size_remaining];

        reader
            .read_exact(&mut reserved)
            .map_err(|e| format!("Error reading reserved buffer: {e}"))?;

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
        // for _color in self.colors.colors.iter() {
        //     println!("{}", _color.get_6bit_color_str());
        // color.lfd_print(indent + 2)
        // }
    }
}

impl Debug for Pltt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
