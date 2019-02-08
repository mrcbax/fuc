pub struct FileDescriptor {
    pub dirty: bool,
    pub block: [u8; 2],
    pub part: [u8; 2],
    pub name: [u8; 12]
}

impl FileDescriptor {
    pub fn new() -> FileDescriptor {
        FileDescriptor{dirty: true, block: [0u8; 2], part: [0u8; 2], name: [0u8; 12]}
    }

    pub fn as_bytes(&self) -> [u8; 16] {
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

    pub fn from_slice(slice: [u8; 16]) -> FileDescriptor {
        FileDescriptor{dirty: true, block: [slice[0], slice[1]], part: [slice[2], slice[3]], name: [slice[4], slice[5], slice[6], slice[7], slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14], slice[15]]}
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_block(&mut self, value: [u8; 2]) {
        self.block = value;
    }
}

impl PartialEq for FileDescriptor {
    fn eq(&self, other: &FileDescriptor) -> bool {
        self.block == other.block
    }
}
