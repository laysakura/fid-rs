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

mod succinct_bit_vector {
    use criterion::{BatchSize, Criterion};
    use succinct_rs::{BitString, SuccinctBitVectorBuilder};

    const NS: [u64; 5] = [1 << 16, 1 << 17, 1 << 18, 1 << 19, 1 << 20];

    pub fn builder_from_length_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!(
                "[{}] SuccinctBitVectorBuilder::from_length(N).build()",
                super::git_hash()
            ),
            |b, &&n| b.iter(|| SuccinctBitVectorBuilder::from_length(n).build()),
            &NS,
        );
    }

    pub fn builder_from_bit_string_benchmark(_: &mut Criterion) {
        super::c().bench_function_over_inputs(
            &format!(
                "[{}] SuccinctBitVectorBuilder::from_bit_string(\"00...(repeated N-times)\").build()",
                super::git_hash()
            ),
            |b, &&n| {
                b.iter_batched(
                    || {
                        let s = String::from_utf8(vec!['0' as u8; n as usize]).unwrap();
                        BitString::new(&s)
                    },
                    |bs| SuccinctBitVectorBuilder::from_bit_string(bs).build(),
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn rank_benchmark(_: &mut Criterion) {
        let times = 1_000_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] SuccinctBitVector::rank(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || SuccinctBitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.rank(n - 1), 0);
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
            &format!(
                "[{}] SuccinctBitVector::select(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let mut builder = SuccinctBitVectorBuilder::from_length(n);
                        for i in 0..n {
                            builder.set_bit(i);
                        }
                        builder.build()
                    },
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.select(n - 1), Some(n - 2));
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
            &format!(
                "[{}] SuccinctBitVector::rank0(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || SuccinctBitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.rank0(n - 1), n);
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
            &format!(
                "[{}] SuccinctBitVector::select0(N) {} times",
                super::git_hash(),
                times
            ),
            move |b, &&n| {
                b.iter_batched(
                    || SuccinctBitVectorBuilder::from_length(n).build(),
                    |bv| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            assert_eq!(bv.select0(n - 1), Some(n - 2));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }
}

mod louds {
    use criterion::{BatchSize, Criterion};
    use succinct_rs::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    const NS: [u64; 5] = [1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15];

    fn generate_binary_tree_lbs(n_nodes: u64) -> BitString {
        assert!(
            NS.iter().any(|n| n - 1 == n_nodes),
            "Only 2^m - 1 nodes (complete binary tree) is supported"
        );

        let mut s = String::from("10");

        // Nodes
        for _ in 1..=(n_nodes / 2) {
            s = format!("{}{}", s, "110");
        }

        // Leaves
        for _ in (n_nodes / 2 + 1)..=(n_nodes) {
            s = format!("{}{}", s, "0");
        }

        BitString::new(&s)
    }

    pub fn builder_from_bit_string_benchmark(_: &mut Criterion) {
        let times = 10;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] LoudsBuilder::from_bit_string(\"...(bin tree of N nodes)\").build() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bs = generate_binary_tree_lbs(n - 1);
                        LoudsBuilder::from_bit_string(bs)
                    },
                    |builder| {
                        for _ in 0..times {
                            builder.build();
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn node_num_to_index_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::node_num_to_index() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bs = generate_binary_tree_lbs(n - 1);
                        LoudsBuilder::from_bit_string(bs).build()
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.node_num_to_index(&LoudsNodeNum::new(n - 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn index_to_node_num_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::index_to_node_num() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bs = generate_binary_tree_lbs(n - 1);
                        LoudsBuilder::from_bit_string(bs).build()
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.index_to_node_num(&LoudsIndex::new(n / 2 + 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn parent_to_children_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::parent_to_children() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bs = generate_binary_tree_lbs(n - 1);
                        LoudsBuilder::from_bit_string(bs).build()
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.parent_to_children(&LoudsNodeNum::new(n - 1));
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
            &NS,
        );
    }

    pub fn child_to_parent_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!(
                "[{}] Louds(N)::child_to_parent() {} times",
                super::git_hash(),
                times,
            ),
            move |b, &&n| {
                b.iter_batched(
                    || {
                        let bs = generate_binary_tree_lbs(n - 1);
                        LoudsBuilder::from_bit_string(bs).build()
                    },
                    |louds| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        for _ in 0..times {
                            let _ = louds.child_to_parent(&LoudsIndex::new(n / 2 + 1));
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
    succinct_bit_vector::builder_from_length_benchmark,
    succinct_bit_vector::builder_from_bit_string_benchmark,
    succinct_bit_vector::rank_benchmark,
    succinct_bit_vector::select_benchmark,
    succinct_bit_vector::rank0_benchmark,
    succinct_bit_vector::select0_benchmark,
    louds::builder_from_bit_string_benchmark,
    louds::node_num_to_index_benchmark,
    louds::index_to_node_num_benchmark,
    louds::parent_to_children_benchmark,
    louds::child_to_parent_benchmark,
);
criterion_main!(benches);
