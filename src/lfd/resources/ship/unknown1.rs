use std::{fmt::Formatter, io::Read};

pub struct Unknown1 {
    pub data: [u8; 3],
}

impl Unknown1 {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let mut data: [u8; 3] = [0; 3];
        reader
            .read_exact(&mut data)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Self { data })
    }
}

impl std::fmt::Debug for Unknown1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("Unknown1 data: 0x{:08X?}", self.data,);
        f.write_str(&debug_string)
    }
}
