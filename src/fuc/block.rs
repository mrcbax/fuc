use super::driver::Writer;
use super::fat::FAT_LENGTH;
use super::volume::MAGIC_LENGTH;

pub const BLOCK_LENGTH: usize = 512;

pub struct Block {
    id: u8,
    data: [u8; BLOCK_LENGTH],
    offset: usize,
    dirty: bool
}

impl Block {
    pub fn new(id: u8) -> Block {
        Block{
            id: id,
            data: [0u8; 512],
            offset: (id as usize * BLOCK_LENGTH) + (FAT_LENGTH + MAGIC_LENGTH),
            dirty: true
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn fill(&mut self, data: [u8; 512]) {
        self.data = data;
        self.dirty = true;
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
        self.dirty = true;
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn clear(&mut self) {
        self.data = [0u8; 512];
        self.dirty = true;
    }

    pub fn write(&mut self, writer: Writer) -> Writer {
        unimplemented!();
        self.dirty = false;
    }
}
