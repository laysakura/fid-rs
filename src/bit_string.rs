/// Provides validated string representation of bit sequence.
///
/// '0' is interpreted as _0_.
/// '1' is interpreted as _1_.
/// '_' is just ignored.
///
/// # Examples
/// ```
/// use succinct_rs::BitString;
///
/// let bs = BitString::new("01");
/// assert_eq!(bs.str(), "01");
///
/// let bs = BitString::new("0111_0101");
/// assert_eq!(bs.str(), "01110101");
/// ```
///
/// # Panics
/// When:
/// - `s` contains any character other than '0', '1', and '_'.
/// - `s` does not contain any '0' or '1'
pub struct BitString {
    s: String,
}

impl BitString {
    /// Constructor.
    pub fn new(s: &str) -> BitString {
        let parsed = s
            .chars()
            .filter(|c| match c {
                '0' => true,
                '1' => true,
                '_' => false,
                _ => panic!("`str` must consist of '0' or '1'. '{}' included.", c),
            })
            .collect::<String>();

        assert!(!parsed.is_empty(), "`str` must contain any '0' or '1'.");

        BitString {
            s: String::from(parsed),
        }
    }

    /// Getter.
    pub fn str(&self) -> &str {
        &self.s
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::super::BitString;

    macro_rules! parameterized_from_valid_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, expected_str) = $value;
                let bvs = BitString::new(in_s);
                assert_eq!(bvs.str(), expected_str);
            }
        )*
        }
    }

    parameterized_from_valid_str_tests! {
        s1: ("0", "0"),
        s2: ("1", "1"),
        s3: ("00", "00"),
        s4: ("01", "01"),
        s5: ("10", "10"),
        s6: ("11", "11"),
        s7_1: ("01010101010111001000001", "01010101010111001000001"),
        s7_2: ("01010101_01011100_1000001", "01010101010111001000001"),
    }
}

#[cfg(test)]
mod new_failure_tests {
    use super::super::BitString;

    macro_rules! parameterized_from_invalid_str_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let in_s = $value;
                let _ = BitString::new(in_s);
            }
        )*
        }
    }

    parameterized_from_invalid_str_tests! {
        s0: "",
        s1: " ",
        s2: " 0",
        s3: "0 ",
        s4: "1 0",
        s5: "０",
        s6: "１",
        s7: "012",
        s8: "01二",
        s9: "_____",
    }
}
