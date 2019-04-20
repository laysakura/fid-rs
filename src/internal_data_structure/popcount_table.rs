/// Cache table of `popcount` results.
pub struct PopcountTable {
    bit_length: u8,

    /// `table[target_num] == target_num.popcount()`
    table: Vec<u8>,
}

impl PopcountTable {
    /// Constructor.
    ///
    /// Time-complexity:  `O(bit_length)` (Assuming `u64::count_ones()` takes `O(1)`)
    /// Space-complexity: `O(bit_length)`
    ///
    /// `bit_length` must be in [1, 64].
    ///
    /// # Panics
    /// When `bit_length` is out of [1, 64].
    pub fn new(bit_length: u8) -> PopcountTable {
        assert!(
            1 <= bit_length && bit_length <= 64,
            "bit_length (= {}) must be in [1, 64]",
            bit_length
        );

        let table = (0..=(1 << bit_length) - 1)
            .map(|target: u64| target.count_ones() as u8)
            .collect();
        PopcountTable { bit_length, table }
    }

    /// Returns the same value as `target.count_ones()` in `O(1)`.
    ///
    /// # Panics
    /// When `target` is out of [0, 2^ `self.bit_length` ).
    pub fn popcount(&self, target: u64) -> u8 {
        assert!(
            target <= ((1 << self.bit_length) - 1),
            "target = {} must be < 2^{}, while PopcountTable::bit_length = {}",
            target,
            self.bit_length,
            self.bit_length
        );

        self.table[target as usize]
    }
}

#[cfg(test)]
mod new_success_tests {
    // well-tested in popcount_success_tests
}

#[cfg(test)]
mod new_failure_tests {
    use super::PopcountTable;

    #[test]
    #[should_panic]
    fn new_0() {
        let _ = PopcountTable::new(0);
    }

    #[test]
    #[should_panic]
    fn new_65() {
        let _ = PopcountTable::new(65);
    }
}

#[cfg(test)]
mod popcount_success_tests {
    use super::PopcountTable;
    use std::ops::RangeInclusive;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let bit_length = $value;
                let tbl = PopcountTable::new(bit_length);

                let range: RangeInclusive<u64> = 0..= ((1 << bit_length) - 1);
                for target in range {
                    assert_eq!(tbl.popcount(target), target.count_ones() as u8);
                }
            }
        )*
        }
    }

    parameterized_tests! {
        bit_length1: 1,
        bit_length2: 2,
        bit_length4: 4,
        bit_length8: 8,
        bit_length16: 16,
        // wants to test 32, 64 but takes too long time

        bit_length15: 15,
        bit_length17: 17,
    }
}

#[cfg(test)]
mod popcount_failure_tests {
    use super::PopcountTable;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let bit_length = $value;
                let tbl = PopcountTable::new(bit_length);
                let _ = tbl.popcount(1 << bit_length);
            }
        )*
        }
    }

    parameterized_tests! {
        bit_length1: 1,
        bit_length2: 2,
        bit_length4: 4,
        bit_length8: 8,
        bit_length16: 16,

        bit_length15: 15,
        bit_length17: 17,
    }
}
