use byteorder::ReadBytesExt;

use crate::lfd::{lfd_header::LfdHeader, traits::lfd_resource::LfdResource};

pub struct Panl {
    pub header: LfdHeader,
    pub raw_data: Vec<u8>,
}

// PANL resources are a special case, and contain a MASK and PLTT after the panel data.
// https://github.com/MikeG621/LfdReader/blob/master/LfdFile.cs
impl LfdResource for Panl {
    fn from_reader(
        reader: &mut std::io::BufReader<std::fs::File>,
        header: LfdHeader,
    ) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut raw_data: Vec<u8> = vec![0; 0];
        let mut still_good = true;

        while still_good {
            match reader.read_u8() {
                Ok(byte) => {
                    raw_data.push(byte);
                }
                Err(_) => {
                    still_good = false;
                }
            }
        }

        Ok(Panl { header, raw_data })
    }

    fn to_writer(&self, writer: &mut dyn std::io::prelude::Write) -> Result<(), String> {
        self.header.to_writer(writer)?;

        writer
            .write(&self.raw_data)
            .map_err(|e| format!("Error writing Panl data: {e}"))?;

        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        todo!()
    }

    fn lfd_get_print_str(&self) -> String {
        todo!()
    }
}
