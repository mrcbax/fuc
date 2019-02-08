use std::fs::File;
use std::fs::OpenOptions;
//use std::process::{Command, Stdio};
use std::io::{BufReader,BufWriter};
use std::io::SeekFrom;
use std::io::prelude::*;
use std::ops::RangeBounds;

pub const MAGIC: [u8; 3] = ['f' as u8, 'u' as u8, 'c' as u8];

use super::block::Block;
use super::file_allocation_table::FileAllocationTable;

use lz4::block::{compress, decompress, CompressionMode};

pub struct Volume {
    pub magic: [u8; 3],
    pub fat: FileAllocationTable,
    pub blocks: Vec<Block>
}

impl Volume {
    pub fn new() -> Volume {
        let blocks: Vec<Block> = Vec::with_capacity(2880);
        Volume{magic: MAGIC, fat: FileAllocationTable::new(), blocks: blocks}
    }
    pub fn create(&mut self) {
        let mut f = OpenOptions::new().write(true).open("/dev/sdb").unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut writer = BufWriter::new(f);
        writer.write(&self.magic).unwrap();
        writer.flush().unwrap();
        self.fat.write().unwrap();
        let mut iter = 0;
        for block in &self.blocks {
            let mut f = OpenOptions::new().write(true).open("/dev/sdb").unwrap();
            f.seek(SeekFrom::Start(46083 + (512 * iter))).unwrap();
            let mut block_writer = BufWriter::new(f);
            block_writer.write(block.data.to_vec().as_slice()).unwrap();
            block_writer.flush().unwrap();
            iter += 1;
        }
    }
    pub fn add_file(&mut self, path: &str) -> Result<usize, & 'static str> {
        let mut f = OpenOptions::new().read(true).open(path).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer: Vec<u8> = vec!();
        f.read_to_end(&mut buffer).unwrap();
        //let comp_buffer: Vec<u8> = vec![0u8; buffer.len()];
        println!("{:?}", buffer);
        let mut comp_bytes = compress(&buffer, Some(CompressionMode::HIGHCOMPRESSION(9)), true).unwrap();
        println!("{:?}", comp_bytes.len());
        pub const OFFSET: usize = 512;
        for i in 0..comp_bytes.len() / 512 {
            let block: Block = Block::new([0, i.clone() as u8]);
            let mut block_bytes: [u8; OFFSET] = [0u8; OFFSET];
            for x in 0..OFFSET {
                if !comp_bytes.is_empty() {
                    block_bytes[x] = comp_bytes.remove(0);
                } else {
                    break;
                }
            }
            self.blocks.push(block.fill(block_bytes));
        }
        Ok(0)
    }
    pub fn write(mut self) -> Result<usize, & 'static str> {
        self.fat.write().unwrap();
        let mut iter: u64 = 0;
        for block in & mut self.blocks {
            if block.is_dirty() {
              let mut f = OpenOptions::new().write(true).open("/dev/sdb").unwrap();
              f.seek(SeekFrom::Start(46083 + (512 * iter))).unwrap();
              let mut block_writer = BufWriter::new(f);
              block_writer.write(block.data.to_vec().as_slice()).unwrap();
              block_writer.flush().unwrap();
              block.write();
            }
            iter += 1;
        }
        Ok(0)
    }
}
