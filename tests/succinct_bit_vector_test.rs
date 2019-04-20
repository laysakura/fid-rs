use succinct_rs::{BitString, SuccinctBitVectorBuilder};

#[test]
fn build_from_length() {
    let bv = SuccinctBitVectorBuilder::from_length(2).build();
    assert_eq!(bv.access(0), false);
    assert_eq!(bv.access(1), false);
}

#[test]
fn build_from_length_and_set_bit() {
    let bv = SuccinctBitVectorBuilder::from_length(2)
        .set_bit(0)
        .set_bit(1)
        .set_bit(0)
        .build();
    assert_eq!(bv.access(0), true);
    assert_eq!(bv.access(1), true);
}

#[test]
fn build_from_bit_string() {
    let bv = SuccinctBitVectorBuilder::from_bit_string(BitString::new("01")).build();
    assert_eq!(bv.access(0), false);
    assert_eq!(bv.access(1), true);
}

#[test]
fn build_from_bit_string_and_set_bit() {
    let bv = SuccinctBitVectorBuilder::from_bit_string(BitString::new("00"))
        .set_bit(0)
        .set_bit(1)
        .set_bit(0)
        .build();
    assert_eq!(bv.access(0), true);
    assert_eq!(bv.access(1), true);
}

#[test]
fn fuzzing_test() {
    let samples = 10000;

    fn access_from_bit_string(s: &str, i: u64) -> bool {
        s.chars().collect::<Vec<char>>()[i as usize] == '1'
    }

    fn rank_from_bit_string(s: &str, i: u64) -> u64 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut rank: u64 = 0;
        for j in 0..=i as usize {
            if chs[j] == '1' {
                rank += 1
            };
        }
        rank
    }

    fn rank0_from_bit_string(s: &str, i: u64) -> u64 {
        let chs = s.chars().collect::<Vec<char>>();
        let mut rank0: u64 = 0;
        for j in 0..=i as usize {
            if chs[j] == '0' {
                rank0 += 1
            };
        }
        rank0
    }

    fn select_from_bit_string(s: &str, num: u64) -> Option<u64> {
        if num == 0 {
            return Some(0);
        }

        let mut cnt: u64 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '1' {
                cnt += 1;
            }
            if cnt == num {
                return Some(i as u64);
            }
        }
        None
    }

    fn select0_from_bit_string(s: &str, num: u64) -> Option<u64> {
        if num == 0 {
            return Some(0);
        }

        let mut cnt: u64 = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == '0' {
                cnt += 1;
            }
            if cnt == num {
                return Some(i as u64);
            }
        }
        None
    }

    for _ in 0..samples {
        let s = &format!("{:b}", rand::random::<u128>());
        eprintln!("build(): bit vec = \"{}\"", s);

        let bs = BitString::new(s);
        let bv = SuccinctBitVectorBuilder::from_bit_string(bs).build();

        for i in 0..s.len() {
            eprintln!("access(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                bv.access(i as u64),
                access_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, SuccinctBitVector::access()={}, access_from_bit_string={}",
                s,
                i,
                bv.access(i as u64),
                access_from_bit_string(s, i as u64)
            );

            eprintln!("rank(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                bv.rank(i as u64),
                rank_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, SuccinctBitVector::rank()={}, rank_from_bit_string={}",
                s,
                i,
                bv.rank(i as u64),
                rank_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                bv.select(num),
                select_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, SuccinctBitVector::select()={:?}, select_from_bit_string={:?}",
                s,
                num,
                bv.select(num),
                select_from_bit_string(s, num)
            );

            eprintln!("rank0(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                bv.rank0(i as u64),
                rank0_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, SuccinctBitVector::rank0()={}, rank0_from_bit_string={}",
                s,
                i,
                bv.rank0(i as u64),
                rank0_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select0(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                bv.select0(num),
                select0_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, SuccinctBitVector::select0()={:?}, select0_from_bit_string={:?}",
                s,
                num,
                bv.select0(num),
                select0_from_bit_string(s, num)
            );
        }
    }
}
