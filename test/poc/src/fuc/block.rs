pub struct Block {
    pub dirty: bool,
    pub id: [u8; 2],
    pub data: [u8; 512]
}

impl Block {
    pub fn new(id: [u8; 2]) -> Block {
        Block{dirty: true, id: id, data: [0u8; 512]}
    }

    pub fn clear(mut self) -> Block {
        self = Block{dirty: true, id: self.id, data: [0u8; 512]};
        self
    }

    pub fn fill(mut self, data: [u8; 512]) -> Block {
        self = Block{dirty: true, id: self.id, data: data};
        self
    }

    pub fn write(& mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
