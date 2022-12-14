use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::resources;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_resource::LfdResource;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;

pub struct Rmap {
    pub header: LfdHeader,
    pub sub_headers: Vec<LfdHeader>,
    pub resources: Vec<Box<dyn LfdResource>>,
}

impl LfdResource for Rmap {
    fn from_reader(reader: &mut BufReader<File>, header: LfdHeader) -> Result<Self, String>
    where
        Self: Sized,
    {
        let num_sub_headers = header.size / 0x10;
        let mut sub_headers: Vec<LfdHeader> = Vec::new();
        let mut resources: Vec<Box<dyn LfdResource>> = Vec::new();

        // parse sub headers
        for _ in 0..num_sub_headers {
            let sub_header = LfdHeader::from_reader(reader)?;
            sub_headers.push(sub_header);
        }

        // parse each sub resource
        for _ in 0..num_sub_headers {
            let resource = resources::create_from_reader(reader)?;
            resources.push(resource);
        }

        Ok(Rmap {
            header,
            sub_headers,
            resources,
        })
    }

    fn to_writer(&self, writer: &mut dyn std::io::Write) -> Result<(), String> {
        self.header.to_writer(writer)?;

        for sub_header in self.sub_headers.iter() {
            sub_header.to_writer(writer)?;
        }

        for resource in self.resources.iter() {
            resource.to_writer(writer)?;
        }

        Ok(())
    }

    fn get_lfd_header(&self) -> &LfdHeader {
        &self.header
    }

    fn lfd_get_print_str(&self) -> String {
        format!("Rmap length[{:?}]", self.resources.len())
    }

    fn lfd_print(&self, indent: usize) {
        let spaces = " ".repeat(indent);
        println!("{spaces}{}", self.lfd_get_print_str());

        for resource in &self.resources {
            resource.lfd_print(indent + INDENT_SIZE);
        }
    }
}

impl Debug for Rmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
