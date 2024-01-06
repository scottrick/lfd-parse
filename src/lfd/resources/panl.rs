use crate::lfd::{lfd_header::LfdHeader, traits::lfd_resource::LfdResource};

pub struct Panl {
    pub header: LfdHeader,
    pub raw_data: Vec<u8>,
}

impl LfdResource for Panl {
    fn from_reader(
        _reader: &mut std::io::BufReader<std::fs::File>,
        _header: LfdHeader,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_writer(&self, _writer: &mut dyn std::io::prelude::Write) -> Result<(), String> {
        todo!()
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        todo!()
    }

    fn lfd_get_print_str(&self) -> String {
        todo!()
    }
}
