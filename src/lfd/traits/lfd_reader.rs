use std::io::Read;
use std::marker::Sized;

pub trait LfdReader {
    fn from_reader(reader: &mut impl Read) -> Result<Self, String>
    where
        Self: Sized;
}
