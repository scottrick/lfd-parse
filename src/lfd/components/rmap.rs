use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::traits::lfd_reader::LfdReader;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::Write;

pub struct Rmap {
    pub header: LfdHeader,
    pub sub_headers: Vec<LfdHeader>,
}

impl LfdReader for Rmap {
    fn from_reader(reader: &mut impl std::io::Read) -> Result<Self, String>
    where
        Self: Sized,
    {
        let header = LfdHeader::from_reader(reader)?;

        let num_sub_headers = header.size / 0x10;
        // let num_sub_headers = num_sub_headers as usize;

        let mut sub_headers: Vec<LfdHeader> = Vec::new();

        for _ in 0..num_sub_headers {
            let sub_header = LfdHeader::from_reader(reader)?;
            sub_headers.push(sub_header);
            // sub_headers[i] = sub_header;
        }

        Ok(Rmap {
            header,
            sub_headers,
        })
    }
}

impl Debug for Rmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "RMAP");
        write!("{:?}", self.header)

        //         for sub_header in &self.sub_headers {
        //             write!("    {:?}", sub_header);
        //         }

        //         Ok(())
    }
}
