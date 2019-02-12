use super::volume::MAGIC_LENGTH;
use super::descriptor::FileDescriptor;
use super::driver::Writer;

pub const FAT_LENGTH: usize = 46080 + MAGIC_LENGTH;

pub struct FileAllocationTable {
    fdesc: Vec<FileDescriptor>,
    dirty: bool
}

impl FileAllocationTable {
    pub fn new() -> FileAllocationTable {
        let fdesc: Vec<FileDescriptor> = vec![FileDescriptor::new(); FAT_LENGTH];
        FileAllocationTable{fdesc: fdesc, dirty: true}
    }

    pub fn as_bytes(&self) -> [u8; FAT_LENGTH] {
        unimplemented!();
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn write(&mut self, writer: Writer) -> Writer {
        unimplemented!();
        self.dirty = false;
    }
}
