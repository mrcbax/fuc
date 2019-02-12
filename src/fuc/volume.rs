use super::fat::FileAllocationTable;
use super::file::File;
use super::driver::Writer;

pub const MAGIC_LENGTH: usize = 3;
pub const MAGIC: [u8; MAGIC_LENGTH] = ['f' as u8, 'u' as u8, 'c' as u8];
pub const VOLUME_LENGTH: usize = 1474560;

pub struct Volume {
    magic: [u8; MAGIC_LENGTH],
    fat: FileAllocationTable,
    files: Vec<File>,
    dirty: bool
}

impl Volume {
    pub fn write(&mut self, writer: Writer) -> Writer {
        unimplemented!();
        self.dirty = false;
    }
}
