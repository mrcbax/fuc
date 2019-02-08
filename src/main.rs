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

//use lz4::block::{compress, decompress, CompressionMode};

fn main() {

    let mut volume: Volume = Volume::new();

    let mut not_done: bool = true;
    let mut curr_block: u8 = 0;
    let mut iter: usize = 0;
    while not_done {
        let mut fdesc = FileDescriptor::new();
        fdesc.set_block([0, curr_block]);
        fdesc.name = ['a' as u8, 'b' as u8, 1,1,1,1,1,1,1,1,1,1];
        volume.fat.add_descriptor(fdesc);
        curr_block += 1;
        iter += 1;
        if iter > 2879 {
            not_done =  false;
        }
    }
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
