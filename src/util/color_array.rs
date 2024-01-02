use core::fmt::Debug;
use std::{fmt::Formatter, fs::File, io::BufReader};

use crate::lfd::traits::lfd_print::LfdPrint;

use super::color::Color;

pub struct ColorArray {
    pub colors: Vec<Color>,
}

impl ColorArray {
    pub fn from_reader(
        reader: &mut BufReader<File>,
        num_colors: usize,
        is_six_bit: bool,
    ) -> Result<Self, String> {
        let mut colors: Vec<Color> = Vec::new();

        for _ in 0..num_colors {
            let new_color = Color::from_reader(reader, is_six_bit)
                .map_err(|e| format!("Error reading color: {e}"))?;

            colors.push(new_color);
        }

        Ok(ColorArray { colors })
    }

    pub fn to_writer(
        &self,
        writer: &mut dyn std::io::Write,
        is_six_bit: bool,
    ) -> Result<(), String> {
        for color in self.colors.iter() {
            color
                .to_writer(writer, is_six_bit)
                .map_err(|e| format!("Error writing color: {e}"))?;
        }

        Ok(())
    }
}

impl Debug for ColorArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("ColorArray [{:?}]", self.colors.len());
        f.write_str(&debug_string)
    }
}

impl LfdPrint for ColorArray {
    fn lfd_get_print_str(&self) -> String {
        format!("{self:?}")
    }
}
