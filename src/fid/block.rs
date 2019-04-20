use super::Block;

impl super::Block {
    /// Constructor.
    pub fn new(value: u16, length: u8) -> Block {
        Block { value, length }
    }

    /// Returns a content (total rank to go) of the block.
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Returns size of the block.
    pub fn length(&self) -> u8 {
        self.length
    }
}
