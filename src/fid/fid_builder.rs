use super::{BitString, Blocks, Chunks, Fid, FidBuilder, FidSeed};
use crate::internal_data_structure::popcount_table::PopcountTable;
use crate::internal_data_structure::raw_bit_vector::RawBitVector;
use std::collections::HashSet;

impl super::FidBuilder {
    /// Prepares a bit vector of `length`, fulfilled with 0.
    pub fn from_length(length: u64) -> Self {
        Self {
            seed: FidSeed::Length(length),
            bits_set: HashSet::new(),
        }
    }

    /// Prepares a bit vector from [BitString](struct.BitString.html) representation.
    pub fn from_bit_string(bs: BitString) -> FidBuilder {
        FidBuilder {
            seed: FidSeed::BitStr(bs),
            bits_set: HashSet::new(),
        }
    }

    /// Set 1 to i-th bit.
    ///
    /// # Panics
    /// When _`i` >= <u>Length of bit vector to build</u>_.
    pub fn set_bit(&mut self, i: u64) -> &mut Self {
        let length = self.current_length();
        assert!(
            i < length,
            "`i` must be smaller than {} (length of bit vector to build)",
            length
        );

        self.bits_set.insert(i);
        self
    }

    /// Add '0' or '1' to current bit vector.
    ///
    /// _WARNING_: Do not use with [from_bit_string()](#method.from_bit_string). It leads to string concatenation and should be too slow.
    pub fn add_bit(&mut self, b: bool) -> &mut Self {
        let length = self.current_length();
        if b {
            self.bits_set.insert(length);
        }
        self.seed = match &self.seed {
            FidSeed::Length(n) => FidSeed::Length(n + 1),
            FidSeed::BitStr(bs) => FidSeed::BitStr(BitString::new(&format!("{}0", bs.str()))),
        };
        self
    }

    /// Build [Fid](struct.Fid.html) in _O(N)_ time (where _N_ is the length of the bit vector to build).
    ///
    /// # Panics
    /// When _`length` == 0_.
    pub fn build(&self) -> Fid {
        assert_ne!(self.current_length(), 0, "length must be > 0.");

        let mut rbv = match &self.seed {
            FidSeed::Length(n) => RawBitVector::from_length(*n),
            FidSeed::BitStr(bs) => RawBitVector::from_bit_string(bs),
        };
        for bit in &self.bits_set {
            rbv.set_bit(*bit)
        }

        let chunks = Chunks::new(&rbv);
        let table = PopcountTable::new(Blocks::calc_block_size(rbv.length()));
        Fid { rbv, chunks, table }
    }

    fn current_length(&self) -> u64 {
        match &self.seed {
            FidSeed::Length(n) => *n,
            FidSeed::BitStr(bs) => bs.str().len() as u64,
        }
    }
}

#[cfg(test)]
mod builder_from_length_success_tests {
    use super::FidBuilder;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_length, index_bit_pairs) = $value;
                let bv = FidBuilder::from_length(in_length).build();
                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (1, vec!(
            IndexBitPair(0, false),
        )),
        t2: (2, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
        )),
        t8: (8, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
        )),
        t9: (9, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1, false),
            IndexBitPair(2, false),
            IndexBitPair(3, false),
            IndexBitPair(4, false),
            IndexBitPair(5, false),
            IndexBitPair(6, false),
            IndexBitPair(7, false),
            IndexBitPair(8, false),
        )),
        t2_pow_16: (1 << 16, vec!(
            IndexBitPair(0, false),
            IndexBitPair((1 << 16) - 1, false),
        )),
        t2_pow_16_p1: ((1 << 16) + 1, vec!(
            IndexBitPair(0, false),
            IndexBitPair(1 << 16, false),
        )),
        t2_pow_16_m1: ((1 << 16) - 1, vec!(
            IndexBitPair(0, false),
            IndexBitPair((1 << 16) - 2, false),
        )),
    }
}

#[cfg(test)]
mod builder_from_length_failure_tests {
    use super::FidBuilder;

    #[test]
    #[should_panic]
    fn empty() {
        let _ = FidBuilder::from_length(0).build();
    }
}
#[cfg(test)]
mod set_bit_success_tests {
    use super::{BitString, FidBuilder};

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, bits_to_set, index_bit_pairs) = $value;
                let mut builder = FidBuilder::from_bit_string(BitString::new(in_s));

                for i in bits_to_set { builder.set_bit(i); }
                let bv = builder.build();

                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("0", vec!(),
               vec!(
                    IndexBitPair(0, false),
                   )),
        t1_2: ("0", vec!(0),
               vec!(
                    IndexBitPair(0, true),
                   )),
        t1_3: ("0", vec!(0, 0),
               vec!(
                    IndexBitPair(0, true),
                   )),
        t1_4: ("1", vec!(0),
               vec!(
                    IndexBitPair(0, true),
                   )),

        t8_1: ("00000000", vec!(),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                    IndexBitPair(2, false),
                    IndexBitPair(3, false),
                    IndexBitPair(4, false),
                    IndexBitPair(5, false),
                    IndexBitPair(6, false),
                    IndexBitPair(7, false),
                   )),
        t8_2: ("00000000", vec!(0, 2, 4, 6),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, false),
                    IndexBitPair(2, true),
                    IndexBitPair(3, false),
                    IndexBitPair(4, true),
                    IndexBitPair(5, false),
                    IndexBitPair(6, true),
                    IndexBitPair(7, false),
                   )),

        t9_1: ("000000000", vec!(),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                    IndexBitPair(2, false),
                    IndexBitPair(3, false),
                    IndexBitPair(4, false),
                    IndexBitPair(5, false),
                    IndexBitPair(6, false),
                    IndexBitPair(7, false),
                    IndexBitPair(8, false),
                   )),
        t9_2: ("000000000", vec!(0, 2, 4, 6, 8),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, false),
                    IndexBitPair(2, true),
                    IndexBitPair(3, false),
                    IndexBitPair(4, true),
                    IndexBitPair(5, false),
                    IndexBitPair(6, true),
                    IndexBitPair(7, false),
                    IndexBitPair(8, true),
                   )),
    }
}

#[cfg(test)]
mod builder_set_bit_failure_tests {
    use super::FidBuilder;

    #[test]
    #[should_panic]
    fn set_bit_over_upper_bound() {
        let _ = FidBuilder::from_length(2).set_bit(2).build();
    }
}

#[cfg(test)]
mod add_bit_success_tests {
    use crate::FidBuilder;

    struct IndexBitPair(u64, bool);

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (init_length, bits_to_add, index_bit_pairs) = $value;
                let mut builder = FidBuilder::from_length(init_length);

                for i in bits_to_add { builder.add_bit(i); }
                let bv = builder.build();

                for IndexBitPair(i, bit) in index_bit_pairs {
                    assert_eq!(bv.access(i), bit);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: (0, vec!(false),
               vec!(
                    IndexBitPair(0, false),
                   )),
        t1_2: (0, vec!(true),
               vec!(
                    IndexBitPair(0, true),
                   )),
        t1_3: (1, vec!(),
               vec!(
                    IndexBitPair(0, false),
                   )),

        t2_1: (0, vec!(false, false),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                   )),
        t2_2: (0, vec!(false, true),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, true),
                   )),
        t2_3: (0, vec!(true, false),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, false),
                   )),
        t2_4: (0, vec!(true, true),
               vec!(
                    IndexBitPair(0, true),
                    IndexBitPair(1, true),
                   )),
        t2_5: (1, vec!(false),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, false),
                   )),
        t2_6: (1, vec!(true),
               vec!(
                    IndexBitPair(0, false),
                    IndexBitPair(1, true),
                   )),
    }
}

#[cfg(test)]
mod builder_add_bit_failure_tests {
    // nothing to test
}
