use std::string::String;

use crate::lfd::components::rmap::Rmap;
use crate::lfd::traits::lfd_reader::LfdReader;

pub struct LfdArchive {
    pub rmap: Rmap,
    // pub lfd_header: LfdHeader,
}

impl LfdReader for LfdArchive {
    fn from_reader(reader: &mut impl std::io::Read) -> Result<Self, String>
    where
        Self: Sized,
    {
        let rmap = Rmap::from_reader(reader)?;

        /*
        let lfd_header = LfdHeader::read_from_buffer(&mut buf_reader)?;

        println!("{file_name}");
        println!("    {lfd_header:?}");

        let num_sub_headers = lfd_header.size / 0x10;

        for _ in 0..num_sub_headers {
            let sub_header = LfdHeader::read_from_buffer(&mut buf_reader)?;
            println!("        {sub_header:?}");
        }

        Ok(LfdFile {
            file_name: file_name.to_string(),
            lfd_header,
        })
            */

        Ok(LfdArchive { rmap })
    }
}
