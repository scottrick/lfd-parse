use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::from_utf8;

#[derive(Default)]
struct LfdHeader {
    lfd_header_type: [u8; 4],
    lfd_header_name: [u8; 8],
    lfd_size: i32,
}

impl Debug for LfdHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            // "TYPE: {:02x?}  NAME: {}, SIZE: {}",
            // self.lfd_header_type,
            "TYPE: [{:02x?} {:02x?} {:02x?} {:02x?}]  NAME: [{}], SIZE: [{}]",
            self.lfd_header_type[0],
            self.lfd_header_type[1],
            self.lfd_header_type[2],
            self.lfd_header_type[3],
            from_utf8(&self.lfd_header_name).unwrap(),
            self.lfd_size
        )
    }
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut file = File::open("data/BATTLE1.LFD").expect("should open");

    for _ in 0..5 {
        let mut header = LfdHeader::default();
        file.read_exact(&mut header.lfd_header_type)?;
        file.read_exact(&mut header.lfd_header_name)?;
        header.lfd_size = file.read_i32::<LittleEndian>()?;
        println!("{header:?}");
    }

    Ok(())
}
