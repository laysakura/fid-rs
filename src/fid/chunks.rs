extern crate rayon;
use rayon::prelude::*;

use super::{Chunk, Chunks};
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Chunks {
    /// Constructor.
    pub fn new(rbv: &RawBitVector) -> Chunks {
        let n = rbv.len();
        let chunk_size: u16 = Chunks::calc_chunk_size(n);
        let chunks_cnt: u64 = Chunks::calc_chunks_cnt(n);

        // In order to use chunks.par_iter_mut(), chunks should have len first.
        // So fill meaning less None value.
        let mut opt_chunks: Vec<Option<Chunk>> = vec![None; chunks_cnt as usize];

        // Parallel - Each chunk has its popcount.
        //     Actually, chunk should have total popcount from index 0 but it is calculated later in sequential manner.
        opt_chunks
            .par_iter_mut()
            .enumerate()
            .for_each(|(i_chunk, chunk)| {
                let this_chunk_size: u16 = if i_chunk as u64 == chunks_cnt - 1 {
                    // When `chunk_size == 6`:
                    //
                    //  000 111 000 11   : rbv
                    // |       |      |  : chunks
                    //
                    // Here, when `i_chunk == 1` (targeting on last '00011' chunk),
                    // `this_chunk_size == 5`
                    let chunk_size_or_0 = (n % chunk_size as u64) as u16;
                    if chunk_size_or_0 == 0 {
                        chunk_size
                    } else {
                        chunk_size_or_0
                    }
                } else {
                    chunk_size
                };

                let chunk_rbv =
                    rbv.clone_sub(i_chunk as u64 * chunk_size as u64, this_chunk_size as u64);

                let popcnt_in_chunk = chunk_rbv.popcount();
                *chunk = Some(Chunk::new(
                    popcnt_in_chunk,
                    this_chunk_size,
                    rbv,
                    i_chunk as u64,
                ));
            });

        // Sequential - Each chunk has total popcount from index 0.
        let mut chunks: Vec<Chunk> = opt_chunks.into_iter().map(|v| v.unwrap()).collect();
        for i_chunk in 0..(chunks_cnt as usize) {
            chunks[i_chunk].value += if i_chunk == 0 {
                0
            } else {
                chunks[i_chunk - 1].value
            }
        }
        Chunks { chunks, chunks_cnt }
    }

    /// Returns size of 1 chunk: _(log N)^2_.
    pub fn calc_chunk_size(n: u64) -> u16 {
        let lg2 = (n as f64).log2() as u16;
        let sz = lg2 * lg2;
        if sz == 0 {
            1
        } else {
            sz
        }
    }

    /// Returns count of chunks: _N / (log N)^2_.
    ///
    /// At max: N / (log N)^2 = 2^64 / 64^2 = 2^(64-12)
    pub fn calc_chunks_cnt(n: u64) -> u64 {
        let chunk_size = Chunks::calc_chunk_size(n);
        n / (chunk_size as u64) + if n % (chunk_size as u64) == 0 { 0 } else { 1 }
    }

    /// Returns i-th chunk.
    ///
    /// # Panics
    /// When _`i` >= `self.chunks_cnt()`_.
    pub fn access(&self, i: u64) -> &Chunk {
        assert!(
            i <= self.chunks_cnt,
            "i = {} must be smaller then {} (self.chunks_cnt())",
            i,
            self.chunks_cnt
        );
        &self.chunks[i as usize]
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::Chunks;
    use crate::internal_data_structure::raw_bit_vector::RawBitVector;

    struct Input<'a> {
        byte_slice: &'a [u8],
        last_byte_len: u8,
        expected_chunk_size: u16,
        expected_chunks: &'a Vec<u64>,
    }

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: Input = $value;
                let rbv = RawBitVector::new(input.byte_slice, 0, input.last_byte_len);
                let n = rbv.len();
                let chunks = Chunks::new(&rbv);

                assert_eq!(Chunks::calc_chunk_size(n), input.expected_chunk_size);
                assert_eq!(Chunks::calc_chunks_cnt(n), input.expected_chunks.len() as u64);
                for (i, expected_chunk) in input.expected_chunks.iter().enumerate() {
                    let chunk = chunks.access(i as u64);
                    assert_eq!(chunk.value(), *expected_chunk);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: Input {
            // N = 1, (log_2(N))^2 = 1
            byte_slice: &[0b0000_0000],
            last_byte_len: 1,
            expected_chunk_size: 1,
            expected_chunks: &vec!(0)
        },
        t2: Input {
            // N = 1, (log_2(N))^2 = 1
            byte_slice: &[0b1000_0000],
            last_byte_len: 1,
            expected_chunk_size: 1,
            expected_chunks: &vec!(1)
        },
        t3: Input {
            // N = 2^2, (log_2(N))^2 = 4
            byte_slice: &[0b0111_0000],
            last_byte_len: 4,
            expected_chunk_size: 4,
            expected_chunks: &vec!(3)
        },
        t4: Input {
            // N = 2^3, (log_2(N))^2 = 9
            byte_slice: &[0b0111_1101],
            last_byte_len: 8,
            expected_chunk_size: 9,
            expected_chunks: &vec!(6)
        },
        t5: Input {
             // N = 2^3 + 1, (log_2(N))^2 = 9
            byte_slice: &[0b0111_1101, 0b1000_0000],
            last_byte_len: 1,
            expected_chunk_size: 9,
            expected_chunks: &vec!(7)
        },
        t6: Input {
            // N = 2^3 + 2, (log_2(N))^2 = 9
            byte_slice: &[0b0111_1101, 0b1100_0000],
            last_byte_len: 2,
            expected_chunk_size: 9,
            expected_chunks: &vec!(7, 8)
        },

        bugfix_11: Input {
            // N = 2^1, (log_2(N))^2 = 4
            byte_slice: &[0b1100_0000],
            last_byte_len: 2,
            expected_chunk_size: 1,
            expected_chunks: &vec!(1, 2)
        },
        bugfix_11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100: Input {
            // N = 8 * 16 = 2^7, (log_2(N))^2 = 49
            byte_slice: &[0b11110110, 0b11010101, 0b01000101, 0b11101111, 0b10101011, 0b10100101, 0b0_1100011, 0b00110100, 0b01010101, 0b10010000, 0b01001100, 0b10111111, 0b00_110011, 0b00111110, 0b01110101, 0b11011100],
            last_byte_len: 8,
            expected_chunk_size: 49,
            expected_chunks: &vec!(30, 53, 72)
        },
    }
}
