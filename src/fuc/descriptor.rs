use super::driver::Writer;

pub const FDESC_LENGTH: usize = 16;

#[derive(Clone)]
pub struct FileDescriptor {
    block_id: [u8; 2],
    part_id: [u8; 2],
    name_id: [u8; 2],
    name: [u8; 12],
    dirty: bool
}

impl FileDescriptor {
    pub fn new() -> FileDescriptor {
        unimplemented!();
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        unimplemented!();
    }

    pub fn set_name(&mut self, name: [u8; 12]) {
        self.name = name;
    }

    pub fn get_name(&self) -> [u8; 12] {
        self.name
    }

    pub fn set_name_id(&mut self, name_id: [u8; 2]) {
        self.name_id = name_id;
    }

    pub fn get_name_id(&self) -> [u8; 2] {
        self.name_id
    }

    pub fn set_part_id(&mut self, part_id: [u8; 2]) {
        self.part_id = part_id;
    }

    pub fn get_part_id(&self) -> [u8; 2] {
        self.part_id
    }

    pub fn set_block_id(&mut self, part_id: [u8; 2]) {
        self.part_id = part_id;
    }

    pub fn get_block_id(&self) -> [u8; 2] {
        self.part_id
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn write(&mut self, writer: Writer) -> Writer {
        unimplemented!();
        self.dirty = false;
    }

}
