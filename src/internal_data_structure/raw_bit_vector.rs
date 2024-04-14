use std::fmt;

#[derive(Debug)]
/// Bit vector of arbitrary length (actually the length is limited to _[1, 2^64)_).
///
/// ```text
/// When fist_byte_offset = 2, last_byte_len = 2:
///
/// 10101010 00000000 11111111
///   |  effective bits |
/// ```
pub struct RawBitVector<'s> {
    byte_slice: &'s [u8],
    first_byte_offset: u8,

    /// Length used in last byte.
    /// Although byte_slice has only 1 byte and first_byte_offset > 0,
    /// this var can take up to 8.
    last_byte_len: u8,
}

impl<'s> RawBitVector<'s> {
    /// Constructor
    ///
    /// # Panics
    /// When:
    /// - `byte_slice` is empty.
    /// - _`first_byte_offset` >= 8_.
    /// - _`last_byte_len` == 0 || `last_byte_len` > 8_.
    /// - _`byte_slice.len() == 1 && first_byte_offset >= last_byte_len`_
    pub fn new(byte_slice: &'s [u8], first_byte_offset: u8, last_byte_len: u8) -> Self {
        assert!(!byte_slice.is_empty());
        assert!(first_byte_offset < 8);
        assert!(0 < last_byte_len && last_byte_len <= 8);
        assert!(!(byte_slice.len() == 1 && first_byte_offset >= last_byte_len));
        Self {
            byte_slice,
            first_byte_offset,
            last_byte_len,
        }
    }

    /// Returns i-th bit.
    ///
    /// ```text
    /// When i=7:
    ///
    ///          |target |
    /// 00000000 01000000
    ///   ^       ^
    /// offset=2  |
    ///  i=0     i=7
    ///        abs_i=9
    ///
    /// abs_i = offset + i
    /// target_byte = at [abs_i / 8]
    /// access(i) = target_byte[abs_i % 8]
    /// ```
    ///
    /// # Panics
    /// When _`i` >= `self.len()`_.
    pub fn access(&self, i: u64) -> bool {
        assert!(i < self.len());

        let abs_i = self.first_byte_offset as u64 + i;
        let byte = self.byte_slice[(abs_i / 8) as usize];
        match abs_i % 8 {
            0 => byte & 0b1000_0000 != 0,
            1 => byte & 0b0100_0000 != 0,
            2 => byte & 0b0010_0000 != 0,
            3 => byte & 0b0001_0000 != 0,
            4 => byte & 0b0000_1000 != 0,
            5 => byte & 0b0000_0100 != 0,
            6 => byte & 0b0000_0010 != 0,
            7 => byte & 0b0000_0001 != 0,
            _ => panic!("never happen"),
        }
    }

    /// Returns length.
    pub fn len(&self) -> u64 {
        if self.byte_slice.len() == 1 {
            self.last_byte_len as u64 - self.first_byte_offset as u64
        } else {
            (self.byte_slice.len() as u64) * 8
                - (self.first_byte_offset as u64)
                - (8 - self.last_byte_len as u64)
        }
    }

    /// Returns popcount of whole this bit vector.
    pub fn popcount(&self) -> u64 {
        let mut popcnt = self
            .byte_slice
            .iter()
            .fold(0, |popcnt: u64, byte| byte.count_ones() as u64 + popcnt);

        // remove 1s in the left of first_byte_offset
        let left_1s_byte = match self.first_byte_offset {
            0 => 0,
            1 => 0b10000000 & self.byte_slice[0],
            2 => 0b11000000 & self.byte_slice[0],
            3 => 0b11100000 & self.byte_slice[0],
            4 => 0b11110000 & self.byte_slice[0],
            5 => 0b11111000 & self.byte_slice[0],
            6 => 0b11111100 & self.byte_slice[0],
            7 => 0b11111110 & self.byte_slice[0],
            _ => panic!("never happen"),
        };
        popcnt -= left_1s_byte.count_ones() as u64;

        // remove 1s in the left of last_byte_len
        let last_byte = self.byte_slice.last().unwrap();
        let last_offset = self.last_byte_len - 1;
        let right_1s_byte = match last_offset {
            0 => 0b01111111 & last_byte,
            1 => 0b00111111 & last_byte,
            2 => 0b00011111 & last_byte,
            3 => 0b00001111 & last_byte,
            4 => 0b00000111 & last_byte,
            5 => 0b00000011 & last_byte,
            6 => 0b00000001 & last_byte,
            7 => 0,
            _ => panic!("never happen"),
        };
        popcnt -= right_1s_byte.count_ones() as u64;

        popcnt
    }

    /// Makes another RawBitVector from _[`i`, `i` + `size`)_ of self.
    /// This method is inexpensive in that it does not copy internal bit vector.
    ///
    /// ```text
    /// offset=2
    ///   |
    ///   v  |     size=14         |
    /// 00000000    00000000    00000000
    ///      ^                    ^
    ///  i_start=3             i_end=16
    ///  abs_i_start=5         abs_i_end=18
    /// | first|                |  last |
    ///
    ///
    /// When i=3 & size=14:
    ///
    /// i_start = 3
    /// abs_i_start = i_start + offset = 5
    /// i_end = i_start + size - 1 = 16
    /// abs_i_end = i_end + offset = 18
    ///
    /// first_byte = at [abs_i_start / 8]
    /// last_byte = at [abs_i_end / 8]
    ///
    /// new_offset = abs_i_start % 8
    ///
    /// new_last_byte_len = abs_i_end % 8 + 1
    /// ```
    ///
    /// # Panics
    /// When:
    /// - _`size` == 0_
    /// - _`size` > `self.len`_
    /// - _`abs_i_end` / 8 + 1 == `self.byte_slice.len()` && abs_i_end` % 8 >= `last_byte_len`_
    pub fn clone_sub(&self, i: u64, size: u64) -> Self {
        assert!(size > 0, "length must be > 0");
        assert!(size <= self.len());

        let i_start = i;
        let abs_i_start = i_start + self.first_byte_offset as u64;
        let i_end = i_start + size - 1;
        let abs_i_end = i_end + self.first_byte_offset as u64;
        assert!(
            abs_i_end / 8 + 1 < self.byte_slice.len() as u64
                || abs_i_end % 8 < self.last_byte_len as u64
        );

        Self {
            byte_slice: &self.byte_slice[(abs_i_start as usize / 8)..=(abs_i_end as usize / 8)],
            first_byte_offset: (abs_i_start % 8) as u8,
            last_byte_len: (abs_i_end % 8 + 1) as u8,
        }
    }

    /// Returns a concatenated number of first 32bits.
    ///
    /// # Panics
    /// If _`self.len()` > 32_
    pub fn as_u32(&self) -> u32 {
        assert!(self.len() <= 32);

        let bs = self.byte_slice;
        let off = self.first_byte_offset;

        assert!(bs.len() <= 5);
        let mut a = [0u32; 5];
        for i in 0..bs.len() {
            a[i] = bs[i] as u32;
        }
        // discard 1s in the last byte
        a[bs.len() - 1] = a[bs.len() - 1] >> (8 - self.last_byte_len) << (8 - self.last_byte_len);

        let mut byte = [0u32; 4];
        for i in 0..4 {
            byte[i] = (a[i] << off) + (a[i + 1] >> (8 - off));
        }

        (byte[0] << 24) | (byte[1] << 16) | (byte[2] << 8) | byte[3]
    }
}

impl<'s> fmt::Display for RawBitVector<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bits_str = self
            .byte_slice
            .iter()
            .enumerate()
            .map(|(i, byte)| {
                let byte_s = format!("{: >8}", format!("{:b}", byte)).replace(' ', "0");
                if i < self.byte_slice.len() - 1 {
                    byte_s
                } else {
                    byte_s
                        .chars()
                        .take(self.last_byte_len as usize)
                        .collect::<String>()
                }
            })
            .collect::<Vec<String>>()
            .concat();

        write!(f, "{}", bits_str)
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len) = $value;
                let _ = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
            }
        )*
        }
    }

    parameterized_tests! {
        t_1byte_1: (&[0b00000000], 0, 8),
        t_1byte_2: (&[0b00000000], 1, 8),
        t_1byte_3: (&[0b00000000], 2, 8),
        t_1byte_4: (&[0b00000000], 3, 8),
        t_1byte_5: (&[0b00000000], 4, 8),
        t_1byte_6: (&[0b00000000], 5, 8),
        t_1byte_7: (&[0b00000000], 6, 8),
        t_1byte_8: (&[0b00000000], 7, 8),
    }
}

#[cfg(test)]
mod new_failure_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len) = $value;
                let _ = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
            }
        )*
        }
    }

    parameterized_tests! {
        t_empty: (&[], 0, 1),
        t_offset: (&[0b00000000], 8, 1),

        t_last_len_0: (&[0b00000000], 0, 0),
        t_last_len_9: (&[0b00000000, 0b00000000], 0, 9),

        t_1byte_1: (&[0b00000000], 0, 9),

        t_1byte_off7: (&[0b00000001], 7, 7),
    }
}

#[cfg(test)]
mod len_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, expected_len) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                assert_eq!(rbv.len(), expected_len);
            }
        )*
        }
    }

    parameterized_tests! {
        t_1byte_off0_1: (&[0b00000000], 0, 8, 8),
        t_1byte_off0_2: (&[0b00000000], 0, 7, 7),
        t_1byte_off0_3: (&[0b00000000], 0, 6, 6),
        t_1byte_off0_4: (&[0b00000000], 0, 5, 5),
        t_1byte_off0_5: (&[0b00000000], 0, 4, 4),
        t_1byte_off0_6: (&[0b00000000], 0, 3, 3),
        t_1byte_off0_7: (&[0b00000000], 0, 2, 2),
        t_1byte_off0_8: (&[0b00000000], 0, 1, 1),

        t_1byte_off1_1: (&[0b00000000], 1, 8, 7),
        t_1byte_off1_2: (&[0b00000000], 1, 7, 6),
        t_1byte_off1_3: (&[0b00000000], 1, 6, 5),
        t_1byte_off1_4: (&[0b00000000], 1, 5, 4),
        t_1byte_off1_5: (&[0b00000000], 1, 4, 3),
        t_1byte_off1_6: (&[0b00000000], 1, 3, 2),
        t_1byte_off1_7: (&[0b00000000], 1, 2, 1),

        t_1byte_off7_1: (&[0b00000000], 7, 8, 1),

        t_2byte_1: (&[0b00000000, 0b00000000], 0, 8, 16),
        t_2byte_2: (&[0b00000000, 0b00000000], 1, 8, 15),
        t_2byte_3: (&[0b00000000, 0b00000000], 7, 8, 9),
        t_2byte_4: (&[0b00000000, 0b00000000], 0, 1, 9),
        t_2byte_5: (&[0b00000000, 0b00000000], 0, 7, 15),
        t_2byte_6: (&[0b00000000, 0b00000000], 7, 1, 2),
    }
}

#[cfg(test)]
mod len_failure_tests {
    // Nothing to do
}

#[cfg(test)]
mod access_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, i, expected_bit) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                assert_eq!(rbv.access(i), expected_bit);
            }
        )*
        }
    }

    parameterized_tests! {
        t_1byte_off0_1: (&[0b10000000], 0, 8, 0, true),

        t_1byte_off1_1: (&[0b01000000], 1, 7, 0, true),
        t_1byte_off1_2: (&[0b01000000], 1, 7, 1, false),

        t_1byte_off7: (&[0b00000001], 7, 8, 0, true),

        t_2byte_1: (&[0b00000000, 0b00000001], 0, 8, 15, true),
        t_2byte_2: (&[0b00000000, 0b00000001], 1, 8, 14, true),
        t_2byte_3: (&[0b00000000, 0b00000001], 7, 8, 8, true),
        t_2byte_4: (&[0b00000000, 0b10000000], 0, 1, 8, true),
        t_2byte_5: (&[0b00000000, 0b00000010], 0, 7, 14, true),
        t_2byte_6: (&[0b00000000, 0b10000000], 7, 1, 1, true),
    }
}

#[cfg(test)]
mod access_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn over_upper_bound() {
        let rbv = RawBitVector::new(&[0b00000000], 1, 2);
        let _ = rbv.access(1);

        // basically, well-tested in len_success_tests
    }
}

#[cfg(test)]
mod popcount_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, expected_popcount) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                assert_eq!(rbv.popcount(), expected_popcount);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (&[0b11111111], 0, 1, 1),
        t2: (&[0b11111111], 1, 8, 7),
        t3: (&[0b11111111], 1, 7, 6),
        t4: (&[0b11111111], 1, 6, 5),
        t5: (&[0b11101111], 0, 8, 7),

        t6: (&[0b01010101, 0b01111111], 0, 1, 4),
        t7: (&[0b10101010, 0b11111111], 0, 1, 5),
        t8: (&[0b11111111, 0b11111111], 0, 1, 9),
        t9: (&[0b11111111, 0b11111111], 1, 1, 8),

        t10: (&[0b11111111, 0b00010000, 0b11111111], 7, 1, 3),
    }
}

#[cfg(test)]
mod popcount_failure_tests {
    // Nothing to do
}

#[cfg(test)]
mod clone_sub_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, i, size, expected_bit_vec) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                let cloned_rbv = rbv.clone_sub(i, size);

                assert_eq!(cloned_rbv.len(), expected_bit_vec.len() as u64);
                for (i, expected_bit) in expected_bit_vec.iter().enumerate() {
                    assert_eq!(cloned_rbv.access(i as u64), *expected_bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: (&[0b01000000], 0, 1, 0, 1, vec![false]),
        t1_2: (&[0b01000000], 1, 2, 0, 1, vec![true]),

        t8_1_1: (&[0b01000101], 0, 8, 0, 1, vec![false]),
        t8_1_2: (&[0b01000101], 0, 8, 0, 2, vec![false, true]),
        t8_1_3: (&[0b01000101], 0, 8, 0, 3, vec![false, true, false]),
        t8_1_4: (&[0b01000101], 0, 8, 0, 4, vec![false, true, false, false]),
        t8_1_5: (&[0b01000101], 0, 8, 0, 5, vec![false, true, false, false, false]),
        t8_1_6: (&[0b01000101], 0, 8, 0, 6, vec![false, true, false, false, false, true]),
        t8_1_7: (&[0b01000101], 0, 8, 0, 7, vec![false, true, false, false, false, true, false]),
        t8_1_8: (&[0b01000101], 0, 8, 0, 8, vec![false, true, false, false, false, true, false, true]),
        t8_1_9: (&[0b01000101, 0b10000000], 1, 1, 0, 8, vec![true, false, false, false, true, false, true, true]),

        t8_2_1: (&[0b01000101], 0, 8, 7, 1, vec![true]),
        t8_2_2: (&[0b01000101, 0b10000000], 1, 1, 6, 2, vec![true, true]),
        t8_2_3: (&[0b01000101, 0b10000000], 1, 1, 7, 1, vec![true]),

        t9_1_1: (&[0b01000101, 0b10000000], 0, 1, 0, 1, vec![false]),
        t9_1_2: (&[0b01000101, 0b10000000], 0, 1, 0, 2, vec![false, true]),
        t9_1_3: (&[0b01000101, 0b10000000], 0, 1, 0, 3, vec![false, true, false]),
        t9_1_4: (&[0b01000101, 0b10000000], 0, 1, 0, 4, vec![false, true, false, false]),
        t9_1_5: (&[0b01000101, 0b10000000], 0, 1, 0, 5, vec![false, true, false, false, false]),
        t9_1_6: (&[0b01000101, 0b10000000], 0, 1, 0, 6, vec![false, true, false, false, false, true]),
        t9_1_7: (&[0b01000101, 0b10000000], 0, 1, 0, 7, vec![false, true, false, false, false, true, false]),
        t9_1_8: (&[0b01000101, 0b10000000], 0, 1, 0, 8, vec![false, true, false, false, false, true, false, true]),
        t9_1_9: (&[0b01000101, 0b10000000], 0, 1, 0, 9, vec![false, true, false, false, false, true, false, true, true]),
        t9_1_10: (&[0b01000101, 0b10000000], 1, 2, 0, 9, vec![true, false, false, false, true, false, true, true, false]),

        t9_2_1: (&[0b01000101, 0b10000000], 0, 1, 7, 1, vec![true]),
        t9_2_2: (&[0b01000101, 0b10000000], 0, 1, 7, 2, vec![true, true]),
        t9_2_3: (&[0b01000101, 0b10000000], 1, 2, 7, 2, vec![true, false]),

        t9_3_1: (&[0b01000101, 0b10000000], 0, 1, 8, 1, vec![true]),
        t9_3_2: (&[0b01000101, 0b10000000], 1, 2, 8, 1, vec![false]),

        t13_1_1: (&[0b10110010, 0b01010000], 0, 4, 9, 3, vec![true, false, true]),
        t13_1_2: (&[0b10110010, 0b01010000], 1, 4, 9, 2, vec![false, true]),

        t_bugfix1: (&[0b11111111, 0b00101001], 0, 1, 0, 1, vec![true]),
    }
}

#[cfg(test)]
mod clone_sub_failure_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, i, size) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                let _ = rbv.clone_sub(i, size);
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: (&[0b00000000], 0, 1, 0, 0),
        t1_2: (&[0b00000000], 0, 1, 0, 2),
        t1_3: (&[0b00000000], 0, 1, 1, 1),
        t1_4: (&[0b00000000], 1, 1, 0, 2),

        t8_1_1: (&[0b01000101], 0, 8, 0, 0),
        t8_1_2: (&[0b01000101], 0, 8, 0, 9),
        t8_1_3: (&[0b01000101, 0b00000000], 1, 1, 0, 9),

        t8_2_1: (&[0b01000101], 0, 8, 7, 0),
        t8_2_2: (&[0b01000101], 0, 8, 7, 2),
        t8_2_3: (&[0b01000101, 0b00000000], 1, 1, 7, 2),

        t9_1_1: (&[0b01000101, 0b00000000], 0, 1, 0, 0),
        t9_1_2: (&[0b01000101, 0b00000000], 0, 1, 0, 10),
        t9_1_3: (&[0b01000101, 0b00000000], 1, 2, 0, 10),

        t9_2_1: (&[0b01000101, 0b00000000], 0, 1, 7, 0),
        t9_2_2: (&[0b01000101, 0b00000000], 0, 1, 7, 3),
        t9_2_3: (&[0b01000101, 0b00000000], 1, 2, 7, 3),

        t9_3_1: (&[0b01000101, 0b00000000], 0, 1, 8, 0),
        t9_3_2: (&[0b01000101, 0b00000000], 0, 1, 8, 2),
        t9_3_3: (&[0b01000101, 0b00000000], 1, 2, 8, 2),
    }
}

#[cfg(test)]
mod clone_sub_fuzzing_tests {
    use super::RawBitVector;

    #[test]
    fn test() {
        let samples = 10000;

        fn sub_str(s: &str, i: u64, size: u64) -> String {
            let ss: String = s.chars().skip(i as usize).take(size as usize).collect();
            ss
        }

        fn str_into_byte_vec(s: &str) -> (Vec<u8>, u8) {
            let bits: Vec<bool> = s.as_bytes().iter().map(|c| *c == '1' as u8).collect();

            let mut byte_vec: Vec<u8> = Vec::with_capacity(bits.len() / 8 + 1);
            let mut last_byte_len = 0u8;

            for bits8 in bits.chunks(8) {
                last_byte_len = bits8.len() as u8; // although this bits8 might not be a last byte.

                let byte = (0..last_byte_len).fold(0, |byte, i| {
                    byte + if bits8[i as usize] { 1 << (7 - i) } else { 0 }
                });
                byte_vec.push(byte);
            }

            (byte_vec, last_byte_len)
        }

        for _ in 0..samples {
            let s = &format!("{:b}", rand::random::<u16>());
            let (byte_vec, last_byte_len) = str_into_byte_vec(s);
            let rbv = RawBitVector::new(&byte_vec[..], 0, last_byte_len);
            // TODO more tests (first_byte_offset > 0)

            for i in 0..s.len() {
                for size in 1..(s.len() - i) {
                    let copied_rbv = rbv.clone_sub(i as u64, size as u64);

                    let substr = sub_str(s, i as u64, size as u64);
                    let (substr_byte_vec, substr_last_byte_len) = str_into_byte_vec(&substr);
                    let substr_rbv =
                        RawBitVector::new(&substr_byte_vec[..], 0, substr_last_byte_len);

                    assert_eq!(copied_rbv.len(), substr_rbv.len());
                    for i in 0..copied_rbv.len() {
                        assert_eq!(
                            copied_rbv.access(i), substr_rbv.access(i),
                            "\nbit vector = {}, RawBitVector::clone_sub(i={}, size={});\nActual:   {}\nExpected: {}",
                            s, i, size, copied_rbv, substr
                        )
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod as_u32_success_tests {
    use super::RawBitVector;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (byte_slice, first_byte_offset, last_byte_len, expected_u32) = $value;
                let rbv = RawBitVector::new(byte_slice, first_byte_offset, last_byte_len);
                assert_eq!(rbv.as_u32(), expected_u32);
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: (&[0b11111111], 0, 1, 0b10000000_00000000_00000000_00000000),
        t1_2: (&[0b11111111], 0, 7, 0b11111110_00000000_00000000_00000000),
        t1_3: (&[0b11111111], 1, 2, 0b10000000_00000000_00000000_00000000),
        t1_4: (&[0b11111111], 1, 7, 0b11111100_00000000_00000000_00000000),

        t8_1: (&[0b10010000], 0, 8, 0b10010000_00000000_00000000_00000000),

        t32_1: (&[0b10010000, 0b01000001, 0b00001000, 0b00011010], 0, 7, 0b10010000_01000001_00001000_00011010),
        t32_2: (&[0b10010000, 0b01000001, 0b00001000, 0b00011010], 0, 8, 0b10010000_01000001_00001000_00011010),
    }
}

#[cfg(test)]
mod as_u32_failure_tests {
    use super::RawBitVector;

    #[test]
    #[should_panic]
    fn test() {
        let byte_slice = &[0b00000000, 0b11111111, 0b00000000, 0b11111111, 0b00000000];
        let rbv = RawBitVector::new(byte_slice, 0, 33);
        // TODO more tests (first_byte_offset > 0)
        let _ = rbv.as_u32();
    }
}
