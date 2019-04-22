use fid_rs::Fid;

#[test]
fn from_str() {
    let fid = Fid::from("01");
    assert_eq!(fid[0], false);
    assert_eq!(fid[1], true);
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

        let fid = Fid::from(s.as_str());

        for i in 0..s.len() {
            eprintln!("[] op: bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid[i as u64],
                access_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Fid::access()={}, access_from_bit_string={}",
                s,
                i,
                fid[i as u64],
                access_from_bit_string(s, i as u64)
            );

            eprintln!("rank(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid.rank(i as u64),
                rank_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Fid::rank()={}, rank_from_bit_string={}",
                s,
                i,
                fid.rank(i as u64),
                rank_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                fid.select(num),
                select_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, Fid::select()={:?}, select_from_bit_string={:?}",
                s,
                num,
                fid.select(num),
                select_from_bit_string(s, num)
            );

            eprintln!("rank0(): bit vec = \"{}\", i = {}, ", s, i);
            assert_eq!(
                fid.rank0(i as u64),
                rank0_from_bit_string(s, i as u64),
                "bit vec = \"{}\", i={}, Fid::rank0()={}, rank0_from_bit_string={}",
                s,
                i,
                fid.rank0(i as u64),
                rank0_from_bit_string(s, i as u64)
            );

            let num = i as u64;
            eprintln!("select0(): bit vec = \"{}\", num = {}, ", s, num);
            assert_eq!(
                fid.select0(num),
                select0_from_bit_string(s, num),
                "bit vec = \"{}\", num={}, Fid::select0()={:?}, select0_from_bit_string={:?}",
                s,
                num,
                fid.select0(num),
                select0_from_bit_string(s, num)
            );
        }
    }
}
