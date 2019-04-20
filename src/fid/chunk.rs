use super::{Blocks, Chunk};
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Chunk {
    /// Constructor.
    pub fn new(value: u64, length: u16, rbv: &RawBitVector, i_chunk: u64) -> Chunk {
        let blocks = Blocks::new(rbv, i_chunk, length);
        Chunk {
            value,
            length,
            blocks,
        }
    }

    /// Returns the content of the chunk.
    pub fn value(&self) -> u64 {
        self.value
    }
}
