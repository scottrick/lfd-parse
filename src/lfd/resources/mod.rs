pub mod rmap;
pub mod unknown;

use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::resources::rmap::Rmap;
use crate::lfd::resources::unknown::Unknown;
use crate::lfd::traits::lfd_resource::LfdResource;

use super::lfd_type::LfdHeaderType;

use std::io::Read;
use std::io::Seek;

pub fn create_from_reader(reader: &mut (impl Read + Seek)) -> Result<Box<dyn LfdResource>, String> {
    let header = LfdHeader::from_reader(reader)?;

    match header.header_type {
        LfdHeaderType::Rmap => {
            let rmap = Rmap::from_reader(reader, header)?;
            Ok(Box::from(rmap))
        }
        _ => {
            let unknown = Unknown::from_reader(reader, header)?;
            Ok(Box::from(unknown))
        }
    }
}
