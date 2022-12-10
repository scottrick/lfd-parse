use crate::lfd::resources::LfdHeader;
use crate::lfd::traits::lfd_print::LfdPrint;
use crate::lfd::traits::lfd_resource::LfdResource;
use core::fmt::Debug;
use core::fmt::Formatter;
use std::io::Read;

pub struct Unknown {
    pub header: LfdHeader,
    pub data: Vec<u8>,
}

impl LfdResource for Unknown {
    fn from_reader(reader: &mut dyn Read, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let size: usize = usize::try_from(header.size).map_err(|e| format!("Invalid size: {e}"))?;
        let mut data: Vec<u8> = vec![0; size];

        reader
            .read_exact(&mut data)
            .map_err(|e| format!("Error reading Unknown buffer: {e}"))?;

        Ok(Unknown { header, data })
    }

    fn to_writer(&self, writer: &mut dyn std::io::Write) -> Result<(), String> {
        self.header.to_writer(writer)?;

        writer
            .write(&self.data)
            .map_err(|e| format!("Error writing Unknown data: {e}"))?;

        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
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
