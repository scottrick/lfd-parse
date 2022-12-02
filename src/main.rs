use byteorder::BigEndian;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io;
use std::io::Read;

mod lfd_header;

use lfd_header::{LfdHeader, LfdHeaderType};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("data/BATTLE2.LFD").expect("should open");

    for _ in 0..5 {
        let mut header = LfdHeader {
            lfd_header_type: file.read_u32::<BigEndian>()?,
            ..Default::default()
        };
        // let mut header = LfdHeader::default();
        // file.read_exact(&mut header.lfd_header_type)?;
        file.read_exact(&mut header.lfd_header_name)?;
        header.lfd_size = file.read_i32::<LittleEndian>()?;
        println!("{header:?}");

        let header_type = LfdHeaderType::from_u32(header.lfd_header_type);
        println!("header type: {header_type:?}");
    }

    Ok(())
}
