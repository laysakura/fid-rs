use super::{Block, Blocks, Chunks};
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Blocks {
    /// Constructor.
    pub fn new(rbv: &RawBitVector, i_chunk: u64, this_chunk_size: u16) -> Blocks {
        let n = rbv.length();
        let chunk_size = Chunks::calc_chunk_size(n);
        let block_size = Blocks::calc_block_size(n);
        let blocks_cnt = this_chunk_size / block_size as u16
            + if this_chunk_size % block_size as u16 == 0 {
                0
            } else {
                1
            };

        let mut blocks: Vec<Block> = Vec::with_capacity(blocks_cnt as usize);
        for i_block in 0..(blocks_cnt as usize) {
            let i_rbv = i_chunk * chunk_size as u64 + i_block as u64 * block_size as u64;
            assert!(i_rbv < n);

            let this_block_size: u8 = if n - i_rbv >= block_size as u64 {
                block_size
            } else {
                (n - i_rbv) as u8
            };

            let block_rbv = rbv.copy_sub(i_rbv, this_block_size as u64);
            let popcount_in_block = block_rbv.popcount() as u16;
            let block = Block::new(
                popcount_in_block
                    + if i_block == 0 {
                        0
                    } else {
                        let block_left = &blocks[i_block - 1];
                        block_left.value()
                    },
                this_block_size,
            );
            blocks.push(block);
        }

        Blocks { blocks, blocks_cnt }
    }

    /// Returns i-th block.
    ///
    /// # Panics
    /// When _`i` >= `self.blocks_cnt()`_.
    pub fn access(&self, i: u64) -> &Block {
        assert!(
            i <= self.blocks_cnt as u64,
            "i = {} must be smaller then {} (self.blocks_cnt())",
            i,
            self.blocks_cnt,
        );
        &self.blocks[i as usize]
    }

    /// Returns size of 1 block: _(log N) / 2_
    pub fn calc_block_size(n: u64) -> u8 {
        let lg2 = (n as f64).log2() as u8;
        let sz = lg2 / 2;
        if sz == 0 {
            1
        } else {
            sz
        }
    }
}
