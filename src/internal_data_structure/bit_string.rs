/// Provides validated string representation of bit sequence.
///
/// '0' is interpreted as _0_.
/// '1' is interpreted as _1_.
/// '_' is just ignored.
///
/// # Examples
/// ```
/// use fid_rs::BitString;
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
pub struct BitString(String);

impl BitString {
    /// Constructor.
    pub fn new(s: &str) -> BitString {
        let parsed: String = s
            .chars()
            .filter(|c| match c {
                '0' | '1' => true,
                '_' => false,
                _ => panic!("`str` must consist of '0' or '1'. '{}' included.", c),
            })
            .collect();
        assert!(!parsed.is_empty(), "`str` must contain any '0' or '1'.");

        BitString(String::from(parsed))
    }

    /// Getter.
    pub fn str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod new_success_tests {
    use super::BitString;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected_str) = $value;
                let bs = BitString::new(s);
                assert_eq!(bs.str(), expected_str);
            }
        )*
        }
    }

    parameterized_tests! {
        s1: ("0", "0"),
        s2: ("1", "1"),
        s3: ("00", "00"),
        s4: ("01", "01"),
        s5: ("10", "10"),
        s6: ("11", "11"),
        s7: ("01010101010111001000001", "01010101010111001000001"),
        s8: ("01010101_01011100_1000001", "01010101010111001000001"),
    }
}

#[cfg(test)]
mod new_failure_tests {
    use super::BitString;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let s = $value;
                let _ = BitString::new(s);
            }
        )*
        }
    }

    parameterized_tests! {
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
