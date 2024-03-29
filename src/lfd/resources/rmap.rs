use crate::lfd::lfd_file::LfdFile;
use crate::lfd::lfd_header::LfdHeader;
use crate::lfd::resources;
use crate::lfd::traits::lfd_print::INDENT_SIZE;
use crate::lfd::traits::lfd_resource::LfdResource;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub struct Rmap {
    pub header: LfdHeader,
    pub sub_headers: Vec<LfdHeader>,
    pub resources: Vec<Box<dyn LfdResource>>,
}

impl Rmap {
    pub fn from_directory(dir: &str, dest: &str, name: &str) -> Result<Self, String> {
        println!("from_directory {dir} to {dest}/{name}.LFD");

        let mut resources: Vec<Box<dyn LfdResource>> = Vec::new();

        for entry in fs::read_dir(dir).map_err(|e| format!("Error reading directory: {e}"))? {
            let entry = entry.map_err(|e| format!("Invalid entry: {e}"))?;
            println!("read entry: {:?}", entry);
            let lfd_file =
                LfdFile::read_from_file(entry.path().to_str().expect("Failed to get file name."))
                    .map_err(|e| format!("Failed to get file name: {}", e))?;
            resources.push(lfd_file.archive.resource);
        }

        for resource in resources.iter() {
            println!("resource: {}", resource.lfd_get_print_str());
        }

        println!("got some resources length: {}", resources.len());
        Err("not done".to_string())
    }
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
            // add/remove resource types to enable/disable debug lfd print
            match resource.get_lfd_header().header_type {
                crate::lfd::lfd_type::LfdHeaderType::Ship => {
                    resource.lfd_print(indent + INDENT_SIZE);
                }
                crate::lfd::lfd_type::LfdHeaderType::Rmap => {
                    resource.lfd_print(indent + INDENT_SIZE);
                }
                _ => {
                    resource.lfd_print(indent + INDENT_SIZE);
                }
            }
        }
    }

    fn expand_to_destination(&self, destination: &str) -> Result<(), String> {
        let _create_dir_result = fs::create_dir(destination);

        for resource in self.resources.iter() {
            resource.expand_to_destination(destination)?;
        }

        Ok(())
    }
}

impl Debug for Rmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lfd_get_print_str())
    }
}
