#![feature(vec_remove_item)]

extern crate lz4;

//use std::fs::File;
//use std::fs::OpenOptions;
//use std::process::{Command, Stdio};
//use std::io::BufWriter;
//use std::io::SeekFrom;
//use std::io::prelude::*;

pub mod fuc;

use fuc::volume::Volume;
use fuc::file_descriptor::FileDescriptor;
use fuc::block::Block;

//use lz4::block::{compress, decompress, CompressionMode};

fn main() {

    let mut volume: Volume = Volume::new();
    let mut fdesc = FileDescriptor::new();
    fdesc.set_block([0, 0]);
    fdesc.name = ['a' as u8, 'b' as u8, 1,1,1,1,1,1,1,1,1,1];
    volume.fat.add_descriptor(fdesc);
    volume.add_file("/home/cbax/Downloads/real_programmers.png").unwrap();
    volume.create();
    println!("volume created");
    std::thread::sleep(std::time::Duration::from_millis(10000));
    volume.blocks[2] = Block{dirty: true, id: volume.blocks[2].id, data: [0u8; 512]};
    volume.write().unwrap();
    println!("block updated");

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

/*let mut child = Command::new("dd")
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

            },
            Err(_) =>{},
        }*/
