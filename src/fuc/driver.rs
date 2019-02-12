use super::volume::VOLUME_LENGTH;

use std::fs::OpenOptions;
use std::io::{BufReader,BufWriter};
use std::io::SeekFrom;
use std::io::prelude::*;

pub struct Device {
    path: String,
    size: usize
}

impl Device {
    pub fn new(path: String) -> Device {
        Device{path: path, size: VOLUME_LENGTH}
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_mut_path(&mut self) -> &mut String {
        &mut self.path
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct Reader {
    device: Device,
    offset: u64
}

impl Reader {
    pub fn read(&self, size: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut f = OpenOptions::new().read(true).open(self.device.get_path()).unwrap();
        f.seek(SeekFrom::Start(self.offset)).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = vec![0u8; size];
        match reader.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(e)
        }
    }

    pub fn set_device(&mut self, device: Device) {
        self.device = device;
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_mut_device(&mut self) -> &mut Device {
        &mut self.device
    }
}

pub struct Writer {
    device: Device,
    offset: u64
}

impl Writer {
    pub fn clear(&self, size: usize) -> Result<usize, std::io::Error> {
        self.write(vec![0u8; size])
    }

    pub fn write(&self, data: Vec<u8>) -> Result<usize, std::io::Error> {
        let mut f = OpenOptions::new().write(true).open(self.device.get_path()).unwrap();
        f.seek(SeekFrom::Start(self.offset)).unwrap();
        let mut writer = BufWriter::new(f);
        writer.write(data.as_slice())
    }

    pub fn set_device(&mut self, device: Device) {
        self.device = device;
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_mut_device(&mut self) -> &mut Device {
        &mut self.device
    }
}
