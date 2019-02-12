use super::block::Block;
use super::descriptor::FileDescriptor;
use super::driver::Writer;

pub struct File {
    desc: FileDescriptor,
    blocks: Vec<Block>,
    dirty: bool
}

impl File {
    pub fn write(&mut self, writer: Writer) -> Writer {
        unimplemented!();
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
