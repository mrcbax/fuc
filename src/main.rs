#![feature(vec_remove_item)]

extern crate lz4;
extern crate rand;

use std::fs::File;
use std::process::{Command, Stdio};
use std::io::Write;

use lz4::block::{compress, decompress, CompressionMode};
use rand::prelude::*;

pub const MAGIC: [u8; 3] = ['f' as u8, 'u' as u8, 'c' as u8];

pub struct FileDescriptor {
    pub dirty: bool,
    pub block: [u8; 2],
    pub part: [u8; 2],
    pub name: [u8; 12]
}

impl FileDescriptor {
    fn new() -> FileDescriptor {
        FileDescriptor{dirty: true, block: [0u8; 2], part: [0u8, 2], name: [0u8, 12]}
    }

    fn as_slice(&self) -> [u8; 16] {
        let mut descriptor_slice: [u8; 16] = [0u8; 16];
        let mut i: usize = 0;
        for byte in &self.block {
            descriptor_slice[i] = byte.clone();
            i += 1;
        }
        for byte in &self.part {
            descriptor_slice[i] = byte.clone();
            i += 1;
        }
        for byte in &self.name {
            descriptor_slice[i] = byte.clone();
            i += 1;
        }
        descriptor_slice
    }

    fn from_slice(slice: [u8; 16]) -> FileDescriptor {
        FileDescriptor{dirty: true, block: [slice[0], slice[1]], part: [slice[2], slice[3]], name: [slice[4], slice[5], slice[6], slice[7], slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14], slice[15]]}
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl PartialEq for FileDescriptor {
    fn eq(&self, other: &FileDescriptor) -> bool {
        self.block == other.block
    }
}

pub struct FileAllocationTable {
    dirty: bool,
    pub fdescs: Vec<FileDescriptor>
}

impl FileAllocationTable {
    fn new() -> FileAllocationTable {
        let fdescs: Vec<FileDescriptor> = Vec::with_capacity(2880);
        FileAllocationTable{dirty: true, fdescs: fdescs}
    }

    fn write(& mut self) -> Result<usize, &'static str> {
        self.dirty = false;
        unimplemented!();
    }

    fn add_descriptor(&mut self, descriptor: FileDescriptor) {
        self.fdescs.push(descriptor);
        self.dirty = true;
    }

    fn remove_descriptor(&mut self, descriptor: &FileDescriptor) {
        self.fdescs.remove_item(descriptor);
        self.dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

pub struct Block {
    pub dirty: bool,
    pub data: [u8; 512]
}

impl Block {
    fn new() -> Block {
        Block{dirty: true, data: [0u8; 512]}
    }

    fn clear(mut self) -> Block {
        self = mut Block{dirty: true, data: [0u8; 512]};
        self
    }

    fn set(mut self, data: [u8; 512]) -> Block {
        self = & mut Block{dirty: true, data: data};
        self
    }

    fn write(& mut self, location: [u8; 2])  -> Result<usize, &'static str> {
        self.dirty = false;
        unimplemented!();
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }
}

pub struct Volume {
    pub magic: [u8; 3],
    pub fat: FileAllocationTable,
    pub blocks: Vec<Block>
}

impl Volume {
    fn new() -> Volume {
        let blocks: Vec<Block> = Vec::with_capacity(2880);
        Volume{magic: MAGIC, fat: FileAllocationTable::new(), blocks: blocks}
    }
    fn create(& mut self) -> Result<usize, & 'static str> {
        let mut child = Command::new("dd")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(["of=/dev/sdb", "bs=3", "count=1"].into_iter())
            .spawn()
            .expect("Failed to spawn child process");

        {
            let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(&self.magic).expect("Failed to write to stdin");
        }

        match child.wait_with_output() {
            Ok(out) => {
                println!("{}", String::from_utf8_lossy(&out.stdout));
                return Ok(0);
            },
            Err(err) => return Err(err.to_string().as_str()),
        }
    }
    fn write(& mut self) -> Result<usize, & 'static str> {
        self.fat.write();
        for block in self.blocks {
            block.write([0u8, 2]);
        }
        unimplemented!();
    }
}

fn main() {

    let mut volume: Volume = Volume::new();
    volume.create();

    /* let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let mut rand_fat: Vec<u8> = vec!();
        for _ in 0..512 {
            let rand_byte: u8 = rng.gen();
            rand_fat.push(rand_byte);
        }
        let comp_wo_prefix = compress(&rand_fat, Some(CompressionMode::HIGHCOMPRESSION(i32::max_value())), false).unwrap();
       // println!("{:?}", &rand_fat);
       // println!("{:?}", &comp_wo_prefix);
        println!("{}", comp_wo_prefix.len());
        assert_eq!(rand_fat, decompress(&comp_wo_prefix, Some(512)).unwrap());
    }*/
}
