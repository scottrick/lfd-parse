use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_reader::LfdReader;

use std::fmt::Debug;
use std::fmt::Formatter;

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
        f.write_str(&self.lfd_get_print_str())
    }
}

impl LfdPrint for Rmap {
    fn lfd_get_print_str(&self) -> String {
        self.header.lfd_get_print_str()
    }

    fn lfd_print(&self, indent: usize) {
        self.header.lfd_print(indent);
        for sub_header in &self.sub_headers {
            sub_header.lfd_print(indent + INDENT_SIZE);
        }
    }
}
