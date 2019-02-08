use std::fs::File;
use std::fs::OpenOptions;
//use std::process::{Command, Stdio};
use std::io::BufWriter;
use std::io::SeekFrom;
use std::io::prelude::*;

use super::file_descriptor::FileDescriptor;

pub struct FileAllocationTable {
    dirty: bool,
    pub fdescs: Vec<FileDescriptor>
}

impl FileAllocationTable {
    pub fn new() -> FileAllocationTable {
        let fdescs: Vec<FileDescriptor> = Vec::with_capacity(2880);
        FileAllocationTable{dirty: true, fdescs: fdescs}
    }

    pub fn write(& mut self) -> Result<usize, &'static str> {
        let mut f = OpenOptions::new().write(true).open("/dev/sdb").unwrap();
        f.seek(SeekFrom::Start(3)).unwrap();
        let mut writer = BufWriter::new(f);
        let bytes = self.as_bytes();
        writer.write(&bytes).unwrap();
        self.dirty = false;
        Ok(0)
    }

    pub fn add_descriptor(&mut self, descriptor: FileDescriptor) {
        self.fdescs.push(descriptor);
        self.dirty = true;
    }

    pub fn remove_descriptor(&mut self, descriptor: &FileDescriptor) {
        self.fdescs.remove_item(descriptor);
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn as_bytes(&self) -> [u8; 46080] {
        let mut bytes: [u8; 46080] = [0u8; 46080];
        let mut ptr: usize = 0;
        for fdesc in &self.fdescs {
            let mut fdesc_ptr = 0;
            for byte in fdesc.as_bytes().iter() {
                bytes[ptr + fdesc_ptr] = byte.clone();
                fdesc_ptr += 1;
            }
            ptr += 16;
        }
        bytes
    }
}
