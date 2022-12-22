use std::fmt::{Debug, Formatter};
use std::io::Read;

use super::lod_header::LodHeader;

pub struct ShipComponent {
    pub lod_headers: Vec<LodHeader>,
}

impl ShipComponent {
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, String> {
        let mut lod_headers: Vec<LodHeader> = Vec::new();

        let mut has_more_headers = true;

        while has_more_headers {
            let header = LodHeader::from_reader(reader)
                .map_err(|e| format!("Error reading LodHeader: {e}"))?;

            if header.distance == 0x7fffffff {
                has_more_headers = false;
            }

            lod_headers.push(header);
        }

        Ok(Self { lod_headers })
    }
}

impl Debug for ShipComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let debug_string = format!("{:?}", self.lod_headers,);
        f.write_str(&debug_string)
    }
}
