use byteorder::BigEndian;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
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

pub struct LfdHeader<'a> {
    pub header_type: LfdHeaderType,
    // pub lfd_header_type: u32,
    pub header_name: &'a str,
    // pub lfd_header_name: [u8; 8],
    pub size: i32,
    // pub lfd_size: i32,
}

impl LfdHeader {
    pub fn read_from_buffer(reader: BufReader<File>) -> Result<Self, String> {
        Ok(LfdHeader {
            header_type: LfdHeaderType::from_u32(reader.read_u32::<BigEndian>()),
            header_name: "asdf",
            size: 1,
        })

        /*
                let mut header = LfdHeader {
            lfd_header_type:
            ..Default::default()
        };
        // let mut header = LfdHeader::default();
        // file.read_exact(&mut header.lfd_header_type)?;
        file.read_exact(&mut header.lfd_header_name)?;
        header.lfd_size = file.read_i32::<LittleEndian>()?;
        */
    }
}

impl Debug for LfdHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TYPE: {:08x?}  NAME: {}, SIZE: {}",
            // self.lfd_header_type,
            // "TYPE: [{:02x?} {:02x?} {:02x?} {:02x?}]  NAME: [{}], SIZE: [{}]",
            // self.lfd_header_type[0],
            // self.lfd_header_type[1],
            // self.lfd_header_type[2],
            // self.lfd_header_type[3],
            self.lfd_header_type,
            from_utf8(&self.lfd_header_name).unwrap(),
            self.lfd_size
        )
    }
}
