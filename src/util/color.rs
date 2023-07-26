use core::fmt::Debug;
use std::{fmt::Formatter, fs::File, io::BufReader};

use byteorder::ReadBytesExt;

use crate::lfd::traits::lfd_print::LfdPrint;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_reader(reader: &mut BufReader<File>, is_six_bit: bool) -> Result<Self, String> {
        let mult = match is_six_bit {
            true => 4u8,
            false => 1u8,
        };

        let r: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading r: {e}"))?
            * mult;
        let g: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading g: {e}"))?
            * mult;
        let b: u8 = reader
            .read_u8()
            .map_err(|e| format!("Error reading b: {e}"))?
            * mult;

        Ok(Color { r, g, b })
    }

    pub fn get_6bit_color_str(&self) -> String {
        format!("{} {} {}\u{0d}", self.r / 4, self.g / 4, self.b / 4)
    }

    pub fn get_8bit_color_str(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!(
            "Color (0x{:02X}, 0x{:02X}, 0x{:02X})",
            self.r, self.g, self.b,
        );
        f.write_str(&debug_string)
    }
}

impl LfdPrint for Color {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
