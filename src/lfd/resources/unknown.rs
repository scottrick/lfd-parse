use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_resource::LfdResource;
use core::fmt::Debug;
use core::fmt::Formatter;
use std::io::Read;
use std::io::Seek;

pub struct Unknown {
    pub header: LfdHeader,
}

impl LfdResource for Unknown {
    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn from_reader(reader: &mut (impl Read + Seek), header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        reader
            .seek(std::io::SeekFrom::Current(header.size.into()))
            .map_err(|e| format!("Error seeking past unknown {e}"))?;

        Ok(Unknown { header })
    }

    fn lfd_get_print_str(&self) -> String {
        self.header.lfd_get_print_str()
    }

    fn lfd_print(&self, indent: usize) {
        self.header.lfd_print(indent);
    }
}

impl Debug for Unknown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
