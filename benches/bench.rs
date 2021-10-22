#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Duration;

fn c() -> Criterion {
    Criterion::default()
        .sample_size(10) // must be >= 10 for Criterion v0.3
        .warm_up_time(Duration::from_secs(1))
        .with_plots()
}

fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

mod fid {
    use criterion::{BatchSize, Criterion};
    use fid_rs::Fid;

    const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

    pub fn from_str_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Fid::from(\"00...(repeated N-times)\")",
                super::git_hash()
            ),
            |b, &&n| {
                b.iter_batched(
                    || String::from_utf8(vec!['0' as u8; n as usize]).unwrap(),
                    |s| Fid::from(s.as_str()),
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn from_slice_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!("[{}] Fid::from(&[false; N])", super::git_hash()),
            |b, &&n| {
                b.iter_batched(
                    || vec![false; n as usize],
                    |v| Fid::from(&v[..]),
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn rank_benchmark(_: &mut Criterion) {
        let times = 1_000_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] Fid::rank(N) {} times", super::git_hash(), times),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let v = vec![false; n as usize];
                        Fid::from(&v[..])
                    },
                    |fid| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(fid.rank1(n - 1), 0);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn select_benchmark(_: &mut Criterion) {
        let times = 1_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] Fid::select(N) {} times", super::git_hash(), times),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let v = vec![true; n as usize];
                        Fid::from(&v[..])
                    },
                    |fid| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(fid.select1(n - 1), Some(n - 2));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn rank0_benchmark(_: &mut Criterion) {
        let times = 1_000_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] Fid::rank0(N) {} times", super::git_hash(), times),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let v = vec![false; n as usize];
                        Fid::from(&v[..])
                    },
                    |fid| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(fid.rank0(n - 1), n);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn select0_benchmark(_: &mut Criterion) {
        let times = 1_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] Fid::select0(N) {} times", super::git_hash(), times),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let v = vec![false; n as usize];
                        Fid::from(&v[..])
                    },
                    |fid| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(fid.select0(n - 1), Some(n - 2));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }
}

criterion_group!(
    benches,
    fid::from_str_benchmark,
    fid::from_slice_benchmark,
    fid::rank_benchmark,
    fid::select_benchmark,
    fid::rank0_benchmark,
    fid::select0_benchmark,
);
criterion_main!(benches);
