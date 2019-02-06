#![feature(vec_remove_item)]

extern crate dd_lib;
extern crate lz4;
extern crate rand;

use lz4::block::{compress, decompress, CompressionMode};
use rand::prelude::*;

pub const MAGIC: [u8; 3] = ['f' as u8, 'u' as u8, 'c' as u8];

pub struct FileDescriptor {
    pub block: [u8; 2],
    pub part: [u8; 2],
    pub name: [u8; 12]
}

impl FileDescriptor {
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
        FileDescriptor{block: [slice[0], slice[1]], part: [slice[2], slice[3]], name: [slice[4], slice[5], slice[6], slice[7], slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14], slice[15]]}
    }
}

impl PartialEq for FileDescriptor {
    fn eq(&self, other: &FileDescriptor) -> bool {
        self.block == other.block
    }
}

pub struct FileAllocationTable {
    pub fdescs: Vec<FileDescriptor>
}

impl FileAllocationTable {
    fn new() -> FileAllocationTable {
        let fdescs: Vec<FileDescriptor> = Vec::with_capacity(2880);
        FileAllocationTable{fdescs: fdescs}
    }

    fn write(&self, location: String) -> Result<usize, &'static str> {
        unimplemented!();
    }

    fn add_descriptor(&mut self, descriptor: FileDescriptor) {
        self.fdescs.push(descriptor);
    }

    fn remove_descriptor(&mut self, descriptor: &FileDescriptor) {
        self.fdescs.remove_item(descriptor);
    }
}

pub struct Block {
    pub data: [u8; 512]
}

impl Block {
    fn new() -> Block {
        Block{data:[0u8; 512]}
    }

    fn clear(& mut self) {
        self = [0u8; 512];
    }

    fn set(& mut self, data: [u8; 512]) {
        self = & mut Block{data: data};
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

}

fn main() {
    let mut rng = rand::thread_rng();
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
    }
}
