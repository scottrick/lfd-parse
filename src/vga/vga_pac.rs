use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::string::String;

use crate::util::color_array::ColorArray;

pub struct VgaPac {
    pub colors: ColorArray,
}

impl VgaPac {
    pub fn read_from_file(file_name: &str) -> Result<Self, String> {
        let file =
            File::open(file_name).map_err(|e| format!("Unable to open file {file_name}: {e}"))?;

        let mut reader = BufReader::new(file);

        let colors = ColorArray::from_reader(&mut reader, 192, true)
            .map_err(|e| format!("Failed to read ColorArray: {e:?}"))?;

        Ok(VgaPac { colors })
    }

    pub fn debug_print_gimp_palette(&self) {
        println!("{:?}", self);
        println!("GIMP Palette");
        println!("Name: VGA.PAC");
        println!("#");
        for color in self.colors.colors.iter() {
            println!("{}", color.get_8bit_color_str());
        }
    }
}

impl Debug for VgaPac {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("VgaPac colors: {:?}", self.colors))
    }
}
