use byteorder::BigEndian;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::from_utf8;

#[derive(Debug)]
pub enum LfdHeaderType {
    Anim,
    Blas,
    Bmap,
    Cust,
    Delt,
    Film,
    Font,
    Gmid,
    Mask,
    Mtrx,
    Panl,
    Pltt,
    Rmap,
    Ship,
    Text,
    Voic,
    Xact,
    Unkn,
}

impl LfdHeaderType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x414E494D => LfdHeaderType::Anim,
            0x424C4153 => LfdHeaderType::Blas,
            0x424D4150 => LfdHeaderType::Bmap,
            0x43555354 => LfdHeaderType::Cust,
            0x44454C54 => LfdHeaderType::Delt,
            0x46494C4D => LfdHeaderType::Film,
            0x464F4E54 => LfdHeaderType::Font,
            0x474D4944 => LfdHeaderType::Gmid,
            0x4D41534B => LfdHeaderType::Mask,
            0x4D545258 => LfdHeaderType::Mtrx,
            0x50414E4C => LfdHeaderType::Panl,
            0x504C5454 => LfdHeaderType::Pltt,
            0x524D4150 => LfdHeaderType::Rmap,
            0x53484950 => LfdHeaderType::Ship,
            0x54455854 => LfdHeaderType::Text,
            0x564F4943 => LfdHeaderType::Voic,
            0x58414354 => LfdHeaderType::Xact,
            _ => LfdHeaderType::Unkn,
        }
    }
}

pub struct LfdHeader {
    pub header_type: LfdHeaderType,
    pub header_name: String,
    pub size: i32,
}

impl LfdHeader {
    pub fn read_from_buffer(reader: &mut BufReader<File>) -> Result<Self, String> {
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
        write!(
            f,
            "LfdHeader[{:?}] Name[{}] Size[{}]",
            self.header_type, self.header_name, self.size
        )
    }
}
