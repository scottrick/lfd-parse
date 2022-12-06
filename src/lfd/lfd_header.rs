use crate::lfd::lfd_type::LfdHeaderType;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_reader::LfdReader;

use byteorder::BigEndian;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::Read;
use std::str::from_utf8;

pub struct LfdHeader {
    pub header_type: LfdHeaderType,
    pub header_name: String,
    pub size: i32,
}

impl LfdReader for LfdHeader {
    fn from_reader(reader: &mut impl Read) -> Result<Self, String>
    where
        Self: Sized,
    {
        let lfd_type = reader
            .read_u32::<BigEndian>()
            .map_err(|e| format!("Error reading lfd_header_type: {e}"))?;

        let mut name: [u8; 8] = [0; 8];
        reader
            .read_exact(&mut name)
            .map_err(|e| format!("Error reading name: {e}"))?;

        let name = from_utf8(&name).map_err(|e| format!("Error reading lfd name: {e}"))?;

        let lfd_size = reader
            .read_i32::<LittleEndian>()
            .map_err(|e| format!("Error reading header_size: {e}"))?;

        Ok(LfdHeader {
            header_type: LfdHeaderType::from_u32(lfd_type),
            header_name: name.to_string(),
            size: lfd_size,
        })
    }
}

impl Debug for LfdHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}

impl LfdPrint for LfdHeader {
    fn lfd_get_print_str(&self) -> String {
        format!(
            "LfdHeader[{:?}] Name[{}] Size[{}]",
            self.header_type, self.header_name, self.size
        )
    }
}
