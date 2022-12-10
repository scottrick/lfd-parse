use crate::lfd::lfd_type::LfdHeaderType;
use crate::lfd::traits::lfd_print::LfdPrint;

use byteorder::BigEndian;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::Read;

pub struct LfdHeader {
    pub header_type: LfdHeaderType,
    pub header_name: String,
    pub size: i32,
}

impl LfdHeader {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String>
    where
        Self: Sized,
    {
        let lfd_type = reader
            .read_u32::<BigEndian>()
            .map_err(|e| format!("Error reading lfd type: {e}"))?;

        let mut name: Vec<u8> = vec![0; 8];
        reader
            .read_exact(&mut name)
            .map_err(|e| format!("Error reading header name: {e}"))?;

        let header_name =
            String::from_utf8(name).map_err(|e| format!("Error reading header name: {e}"))?;

        let size = reader
            .read_i32::<LittleEndian>()
            .map_err(|e| format!("Error reading header size: {e}"))?;

        Ok(LfdHeader {
            header_type: LfdHeaderType::from(lfd_type),
            header_name,
            size,
        })
    }

    pub fn to_writer(&self, writer: &mut dyn std::io::Write) -> Result<(), String> {
        writer
            .write_u32::<BigEndian>(self.header_type.to_u32())
            .map_err(|e| format!("Error writing lfd type: {e}"))?;

        let name_clone = self.header_name.clone().into_bytes();
        writer
            .write_all(&name_clone)
            .map_err(|e| format!("Error writing header name: {e}"))?;

        writer
            .write_i32::<LittleEndian>(self.size)
            .map_err(|e| format!("Error writing header size: {e}"))?;

        Ok(())
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
            "LfdHeader[{:?}] name[{}] size[{}]",
            self.header_type, self.header_name, self.size
        )
    }
}
