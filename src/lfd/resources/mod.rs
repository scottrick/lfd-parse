pub mod delt;
pub mod rmap;
pub mod ship;
pub mod unknown;

use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::resources::rmap::Rmap;
use crate::lfd::resources::unknown::Unknown;
use crate::lfd::traits::lfd_resource::LfdResource;

use self::delt::Delt;
use self::ship::Ship;

use super::lfd_type::LfdHeaderType;

use std::io::Read;

pub fn create_from_reader(reader: &mut dyn Read) -> Result<Box<dyn LfdResource>, String> {
    let header = LfdHeader::from_reader(reader)?;

    match header.header_type {
        LfdHeaderType::Delt(_) => {
            let delt = Delt::from_reader(reader, header)?;
            Ok(Box::from(delt))
        }
        LfdHeaderType::Rmap(_) => {
            let rmap = Rmap::from_reader(reader, header)?;
            Ok(Box::from(rmap))
        }
        LfdHeaderType::Ship(_) => {
            let ship = Ship::from_reader(reader, header)?;
            Ok(Box::from(ship))
        }
        _ => {
            let unknown = Unknown::from_reader(reader, header)?;
            Ok(Box::from(unknown))
        }
    }
}
