use super::{Blocks, Chunks, Fid};
use crate::internal_data_structure::bit_string::BitString;
use crate::internal_data_structure::popcount_table::PopcountTable;
use crate::internal_data_structure::raw_bit_vector::RawBitVector;
use std::ops::Index;

impl From<&str> for Fid {
    /// Constructor from string representation of bit sequence.
    ///
    /// - '0' is interpreted as _0_.
    /// - '1' is interpreted as _1_.
    /// - '_' is just ignored.
    ///
    /// # Examples
    /// ```
    /// use fid_rs::Fid;
    ///
    /// let fid = Fid::from("01_11");
    /// assert_eq!(fid[0], false);
    /// assert_eq!(fid[1], true);
    /// assert_eq!(fid[2], true);
    /// assert_eq!(fid[3], true);
    /// ```
    ///
    /// # Panics
    /// When:
    /// - `s` contains any character other than '0', '1', and '_'.
    /// - `s` does not contain any '0' or '1'
    fn from(s: &str) -> Self {
        let bs = BitString::new(s);
        let bits: Vec<bool> = bs
            .str()
            .as_bytes()
            .iter()
            .map(|c| *c == '1' as u8)
            .collect();
        Self::from(&bits[..])
    }
}

impl From<&[bool]> for Fid {
    /// Constructor from slice of boolean.
    ///
    /// # Examples
    /// ```
    /// use fid_rs::Fid;
    ///
    /// let bits = [false, true, true, true];
    /// let fid = Fid::from(&bits[..]);
    /// assert_eq!(fid[0], false);
    /// assert_eq!(fid[1], true);
    /// assert_eq!(fid[2], true);
    /// assert_eq!(fid[3], true);
    /// ```
    ///
    /// # Panics
    /// When:
    /// - `bits` is empty.
    fn from(bits: &[bool]) -> Self {
        assert!(!bits.is_empty());

        let mut byte_vec: Vec<u8> = Vec::with_capacity(bits.len() / 8 + 1);
        let mut last_byte_len = 0u8;

        for bits8 in bits.chunks(8) {
            last_byte_len = bits8.len() as u8; // although this bits8 might not be a last byte.

            let byte = (0..last_byte_len).fold(0, |byte, i| {
                byte + if bits8[i as usize] { 1 << (7 - i) } else { 0 }
            });
            byte_vec.push(byte);
        }

        Fid::build(byte_vec, last_byte_len)
    }
}

static TRUE: bool = true;
static FALSE: bool = false;

impl Index<u64> for Fid {
    type Output = bool;

    /// Returns `i`-th element of the `Fid`.
    ///
    /// # Panics
    /// When _`i` >= length of the `Fid`_.
    fn index(&self, index: u64) -> &Self::Output {
        if self.rbv().access(index) {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl Fid {
    /// Build FID from byte vector.
    fn build(byte_vec: Vec<u8>, last_byte_len: u8) -> Self {
        let bit_len = (byte_vec.len() - 1) as u64 * 8 + last_byte_len as u64;
        let rbv = RawBitVector::new(&byte_vec[..], 0, last_byte_len);
        let chunks = Chunks::new(&rbv);
        let table = PopcountTable::new(Blocks::calc_block_size(rbv.len()));
        Self {
            byte_vec,
            bit_len,
            chunks,
            table,
        }
    }

    /// Returns the number of _1_ in _[0, `i`]_ elements of the `Fid`.
    ///
    /// # Panics
    /// When _`i` >= length of the `Fid`_.
    ///
    /// # Implementation detail
    ///
    /// ```text
    ///  00001000 01000001 00000100 11000000 00100000 00000101 00100000 00010000 001  Raw data (N=67)
    ///                                                           ^
    ///                                                           i = 51
    /// |                  7                    |                12                |  Chunk (size = (log N)^2 = 36)
    ///                                         ^
    ///                chunk_left            i_chunk = 1      chunk_right
    ///
    /// |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |3 |3 |4  |4 |4 |5  |5|  Block (size = log N / 2 = 3)
    ///                                                         ^
    ///                                                      i_block = 17
    ///                                              block_left | block_right
    /// ```
    ///
    /// 1. Find `i_chunk`. _`i_chunk` = `i` / `chunk_size`_.
    /// 2. Get _`chunk_left` = Chunks[`i_chunk` - 1]_ only if _`i_chunk` > 0_.
    /// 3. Get _rank from chunk_left_ if `chunk_left` exists.
    /// 4. Get _`chunk_right` = Chunks[`i_chunk`]_.
    /// 5. Find `i_block`. _`i_block` = (`i` - `i_chunk` * `chunk_size`) / block size_.
    /// 6. Get _`block_left` = `chunk_right.blocks`[ `i_block` - 1]`_ only if _`i_block` > 0_.
    /// 7. Get _rank from block_left_ if `block_left` exists.
    /// 8. Get inner-block data _`block_bits`. `block_bits` must be of _block size_ length, fulfilled with _0_ in right bits.
    /// 9. Calculate _rank of `block_bits`_ in _O(1)_ using a table memonizing _block size_ bit's popcount.
    pub fn rank(&self, i: u64) -> u64 {
        let n = self.len();
        assert!(i < n);
        let chunk_size = Chunks::calc_chunk_size(n);
        let block_size = Blocks::calc_block_size(n);

        // 1.
        let i_chunk = i / chunk_size as u64;

        // 3.
        let rank_from_chunk = if i_chunk == 0 {
            0
        } else {
            // 2., 3.
            let chunk_left = self.chunks.access(i_chunk - 1);
            chunk_left.value()
        };

        // 4.
        let chunk_right = self.chunks.access(i_chunk);

        // 5.
        let i_block = (i - i_chunk * chunk_size as u64) / block_size as u64;

        // 7.
        let rank_from_block = if i_block == 0 {
            0
        } else {
            // 6., 7.
            let block_left = chunk_right.blocks.access(i_block - 1);
            block_left.value()
        };

        // 8.
        let block_right = chunk_right.blocks.access(i_block);
        let pos_block_start = i_chunk * chunk_size as u64 + i_block * block_size as u64;
        assert!(i - pos_block_start < block_right.length() as u64);
        let block_right_rbv = self
            .rbv()
            .clone_sub(pos_block_start, block_right.length() as u64);
        let block_right_as_u32 = block_right_rbv.as_u32();
        let bits_to_use = i - pos_block_start + 1;
        let block_bits = block_right_as_u32 >> (32 - bits_to_use);
        let rank_from_table = self.table.popcount(block_bits as u64);

        // 9.
        rank_from_chunk + rank_from_block as u64 + rank_from_table as u64
    }

    /// Returns the number of _0_ in _[0, `i`]_ elements of the `Fid`.
    ///
    /// # Panics
    /// When _`i` >= length of the `Fid`_.
    pub fn rank0(&self, i: u64) -> u64 {
        (i + 1) - self.rank(i)
    }

    /// Returns the minimum position (0-origin) `i` where _`rank(i)` == num_ of `num`-th _1_ if exists. Else returns None.
    ///
    /// # Panics
    /// When _`num` > length of the `Fid`_.
    ///
    /// # Implementation detail
    /// Binary search using `rank()`.
    pub fn select(&self, num: u64) -> Option<u64> {
        let n = self.len();
        assert!(num <= n);

        if num == 0 || num == 1 && self[0] == true {
            return Some(0);
        }
        if self.rank(n - 1) < num {
            return None;
        };

        let mut ng = 0;
        let mut ok = n - 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.rank(mid) >= num {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        Some(ok)
    }

    /// Returns the minimum position (0-origin) `i` where _`rank(i)` == num_ of `num`-th _0_ if exists. Else returns None.
    ///
    /// # Panics
    /// When _`num` > length of the `Fid`_.
    pub fn select0(&self, num: u64) -> Option<u64> {
        let n = self.bit_len;
        assert!(num <= n);

        if num == 0 || num == 1 && self[0] == false {
            return Some(0);
        }
        if self.rank0(n - 1) < num {
            return None;
        };

        let mut ng = 0;
        let mut ok = n - 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.rank0(mid) >= num {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        Some(ok)
    }

    /// Returns bit length of this FID.
    pub fn len(&self) -> u64 {
        self.bit_len
    }

    fn rbv(&self) -> RawBitVector {
        let last_byte_len_or_0 = (self.bit_len % 8) as u8;
        RawBitVector::new(
            &self.byte_vec[..],
            0,
            if last_byte_len_or_0 == 0 {
                8
            } else {
                last_byte_len_or_0
            },
        )
    }
}

#[cfg(test)]
mod from_str_success_tests {
    use crate::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected_bits) = $value;
                let fid = Fid::from(s);

                // TODO length check
                // assert_eq!(fid.length(), expected_bits);
                for (i, bit) in expected_bits.iter().enumerate() {
                    assert_eq!(fid[i as u64], *bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: ("0", vec![false]),
        t2: ("1", vec![true]),
        t3: ("00", vec![false, false]),
        t4: ("01", vec![false, true]),
        t5: ("10", vec![true, false]),
        t6: ("11", vec![true, true]),
        t7: ("0101_0101__0101_1100__1000_001", vec![
            false, true, false, true,
            false, true, false, true,
            false, true, false, true,
            true, true, false, false,
            true, false, false, false,
            false, false, true,
        ]),
    }
}

#[cfg(test)]
mod from_str_failure_tests {
    // well-tested in BitString::new()
}

#[cfg(test)]
mod from_slice_success_tests {
    use crate::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let arr = $value;
                let fid = Fid::from(&arr[..]);

                // TODO length check
                // assert_eq!(fid.length(), expected_bits);
                for (i, bit) in arr.iter().enumerate() {
                    assert_eq!(fid[i as u64], *bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: [false],
        t2: [true],
        t3: [false, false],
        t4: [false, true],
        t5: [true, false],
        t6: [true, true],
        t7: [false; 100],
        t8: [true; 100],
    }
}

#[cfg(test)]
mod from_slice_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn empty() {
        Fid::from(&[][..]);
    }
}

#[cfg(test)]
mod index_u64_success_tests {
    // well-tested in fid_builder::{builder_from_length_success_tests, builder_from_bit_string_success_tests}
}

#[cfg(test)]
mod index_u64_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn over_upper_bound() {
        let fid = Fid::from("00");
        let _ = fid[2];
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank_success_tests {
    use crate::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_fid_str, in_i, expected_rank) = $value;
                assert_eq!(
                    Fid::from(in_fid_str).rank(in_i),
                    expected_rank
                );
            }
        )*
        }
    }

    parameterized_tests! {
        rank1_1: ("0", 0, 0),

        rank2_1: ("00", 0, 0),
        rank2_2: ("00", 1, 0),

        rank3_1: ("01", 0, 0),
        rank3_2: ("01", 1, 1),

        rank4_1: ("10", 0, 1),
        rank4_2: ("10", 1, 1),

        rank5_1: ("11", 0, 1),
        rank5_2: ("11", 1, 2),

        rank6_1: ("10010", 0, 1),
        rank6_2: ("10010", 1, 1),
        rank6_3: ("10010", 2, 1),
        rank6_4: ("10010", 3, 2),
        rank6_5: ("10010", 4, 2),

        bugfix_11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100: (
            "11110110_11010101_01000101_11101111_10101011_10100101_01100011_00110100_01010101_10010000_01001100_10111111_00110011_00111110_01110101_11011100",
            49, 31,
        ),
        bugfix_10100001_01010011_10101100_11100001_10110010_10000110_00010100_01001111_01011100_11010011_11110000_00011010_01101111_10101010_11000111_0110011: (
            "10100001_01010011_10101100_11100001_10110010_10000110_00010100_01001111_01011100_11010011_11110000_00011010_01101111_10101010_11000111_0110011",
            111, 55,
        ),
        bugfix_100_111_101_011_011_100_101_001_111_001_001_101_100_011_000_111_1___01_000_101_100_101_101_001_011_110_010_001_101_010_010_010_111_111_111_001_111_001_100_010_001_010_101_11: (
            "100_111_101_011_011_100_101_001_111_001_001_101_100_011_000_111_1___01_000_101_100_101_101_001_011_110_010_001_101_010_010_010_111_111_111_001_111_001_100_010_001_010_101_11",
            48, 28,
        ),
        bugfix_11100100_10110100_10000000_10111111_01110101_01100110_00101111_11101001_01100100_00001000_11010100_10100000_00010001_10100101_01100100_0010010: (
            "11100100_10110100_10000000_10111111_01110101_01100110_00101111_11101001_01100100_00001000_11010100_10100000_00010001_10100101_01100100_0010010",
            126, 56,
        ),
    }
    // Tested more in tests/ (integration test)
}

#[cfg(test)]
mod rank_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn rank_over_upper_bound() {
        let fid = Fid::from("00");
        let _ = fid.rank(2);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod rank0_success_tests {
    use crate::Fid;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_fid_str, in_i, expected_rank0) = $value;
                assert_eq!(
                    Fid::from(in_fid_str).rank0(in_i),
                    expected_rank0
                );
            }
        )*
        }
    }

    parameterized_tests! {
        rank0_1_1: ("0", 0, 1),

        rank0_2_1: ("00", 0, 1),
        rank0_2_2: ("00", 1, 2),

        rank0_3_1: ("01", 0, 1),
        rank0_3_2: ("01", 1, 1),

        rank0_4_1: ("10", 0, 0),
        rank0_4_2: ("10", 1, 1),

        rank0_5_1: ("11", 0, 0),
        rank0_5_2: ("11", 1, 0),

        rank0_6_1: ("10010", 0, 0),
        rank0_6_2: ("10010", 1, 1),
        rank0_6_3: ("10010", 2, 2),
        rank0_6_4: ("10010", 3, 2),
        rank0_6_5: ("10010", 4, 3),
    }
    // Tested more in tests/ (integration test)
}

#[cfg(test)]
mod rank0_0_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn rank0_over_upper_bound() {
        let fid = Fid::from("00");
        let _ = fid.rank0(2);
    }
}

#[cfg(test)]
mod select_success_tests {
    // Tested well in tests/ (integration test)
}

#[cfg(test)]
mod select_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn select_over_max_rank() {
        let fid = Fid::from("00");
        let _ = fid.select(3);
    }
}

#[cfg(test)]
mod select0_success_tests {
    // Tested well in tests/ (integration test)
}

#[cfg(test)]
mod select0_failure_tests {
    use crate::Fid;

    #[test]
    #[should_panic]
    fn select_over_max_rank() {
        let fid = Fid::from("00");
        let _ = fid.select0(3);
    }
}
