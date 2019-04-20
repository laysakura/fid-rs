# Succinct.rs

Succinct Data Structures library for Rust.

[Master API Docs](https://laysakura.github.io/succinct.rs/succinct_rs/)
|
[Released API Docs](https://docs.rs/crate/succinct_rs)
|
[Benchmark Results](https://laysakura.github.io/succinct.rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/succinct.rs/blob/master/CHANGELOG.md)

[![Build Status](https://travis-ci.com/laysakura/succinct.rs.svg?branch=master)](https://travis-ci.com/laysakura/succinct.rs)
[![Crates.io](https://img.shields.io/crates/v/succinct_rs.svg)](https://crates.io/crates/succinct_rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/succinct.rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/succinct.rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/succinct.rs/blob/master/LICENSE-APACHE)

Succinct.rs is a library to provide succinct data structures with _simple API_ and _high performance_.

Currently, **[Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html)** and **[LOUDS (Level-Order Unary Degree Sequence)](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html)** are supported.

## Table of Contents
- [Table of Contents](#table-of-contents)
  - [Quickstart](#quickstart)
  - [Features](#features)
  - [Versions](#versions)
  - [Roadmap](#roadmap)
  - [Contributing](#contributing)
  - [License](#license)

## Quickstart

To use with Succinct.rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
succinct_rs = "0.6"
```

### [Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html) Usage

```rust
extern crate succinct_rs;

use succinct_rs::{BitString, SuccinctBitVectorBuilder};

// Construction -------------------------
// `01001` built by `from_bit_string()`
let bv = SuccinctBitVectorBuilder::from_bit_string(BitString::new("0100_1")).build();  // Tips: BitString::new() ignores '_'.

// `01001` built by `from_length()` and `add_bit()`
let bv = SuccinctBitVectorBuilder::from_length(0)
    .add_bit(false)
    .add_bit(true)
    .add_bit(false)
    .add_bit(false)
    .add_bit(true)
    .build();

// Basic operations ---------------------
assert_eq!(bv.access(0), false);  // [0]1001; 0th bit is '0' (false)
assert_eq!(bv.access(1), true);   // 0[1]001; 1st bit is '1' (true)
assert_eq!(bv.access(4), true);   // 0100[1]; 4th bit is '1' (true)

assert_eq!(bv.rank(0), 0);  // [0]1001; Range [0, 0] has no '1'
assert_eq!(bv.rank(3), 1);  // [0100]1; Range [0, 3] has 1 '1'
assert_eq!(bv.rank(4), 2);  // [01001]; Range [0, 4] has 2 '1's

assert_eq!(bv.select(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '1's is i=0
assert_eq!(bv.select(1), Some(1)); // 0[1]001; Minimum i where range [0, i] has 1 '1's is i=1
assert_eq!(bv.select(2), Some(4)); // 0100[1]; Minimum i where range [0, i] has 2 '1's is i=4
assert_eq!(bv.select(3), None);    // There is no i where range [0, i] has 3 '1's

// rank0, select0 -----------------------
assert_eq!(bv.rank0(0), 1);  // [0]1001; Range [0, 0] has no '0'
assert_eq!(bv.rank0(3), 3);  // [0100]1; Range [0, 3] has 3 '0's
assert_eq!(bv.rank0(4), 3);  // [01001]; Range [0, 4] has 3 '0's

assert_eq!(bv.select0(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '0's is i=0
assert_eq!(bv.select0(1), Some(0)); // [0]1001; Minimum i where range [0, i] has 1 '0's is i=0
assert_eq!(bv.select0(2), Some(2)); // 01[0]01; Minimum i where range [0, i] has 2 '0's is i=2
assert_eq!(bv.select0(4), None);    // There is no i where range [0, i] has 4 '0's
```

### [LOUDS](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.Louds.html) Usage

```rust
extern crate succinct_rs;

use succinct_rs::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

// Construct from LBS.
let bs = BitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0");
let louds = LoudsBuilder::from_bit_string(bs).build();

// LoudsNodeNum <-> LoudsIndex
let node8 = LoudsNodeNum::new(8);
let index11 = louds.node_num_to_index(&node8);
assert_eq!(louds.index_to_node_num(&index11), node8);

// Search for children.
assert_eq!(louds.parent_to_children(&node8), vec!(LoudsIndex::new(17), LoudsIndex::new(18)));

// Search for parent.
assert_eq!(louds.child_to_parent(&index11), LoudsNodeNum::new(4));
```

## Features

- **Arbitrary length support with minimum working memory**: Succinct.rs provides virtually _arbitrary size_ of data structures. There are carefully designed to use as small memory space as possible.
- **Simple public APIs**: Each data structures almost only have very basic operations for the data structure. `succinct::SuccinctBitVector`, for example, has only `access()`, `rank()`, and `select()`.
- **Latest benchmark results are always accessible**: Succinct.rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/succinct.rs/criterion/report/).

### [Succinct Bit Vector](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html) Complexity

When the length of a `SuccinctBitVector` is _N_:

|                  | [build()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVectorBuilder.html#method.build) | [access()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html#method.access) | [rank()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html#method.rank) | [select()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html#method.select) |
|------------------|--------------------------------------------------------|------------|----------|------------|
| Time-complexity  | _O(N)_                                                 | _O(1)_     | _O(1)_   | _O(log N)_ |
| Space-complexity | _N + o(N)_                                             | _0_        | _O(log N)_   | _O(log N)_     |

(Actually, `select()`'s time-complexity can be _O(1)_ with complex implementation but Succinct.rs, like many other libraries, uses binary search of `rank()`'s result).

### [LOUDS](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html) Complexity

When the number of nodes in the tree represented as LOUDS is _N_:

|                  | [build()](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.LoudsBuilder.html#method.build) | [node_num_to_index()](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html#method.node_num_to_index) | [index_to_node_num()](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html#method.index_to_node_num) | [child_to_parent()](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html#method.child_to_parent) | [parent_to_children()](https://laysakura.github.io/succinct.rs/succinct_rs/louds/struct.Louds.html#method.parent_to_children) |
|------------------|--------------------------------------------------------|------------|----------|------------|----|
| Time-complexity  | _O(N)_                                                 | _O(log N)_     | _O(1)_   | _O(1)_ | _O( max(log N, <u>max num of children a node has</u>) )_ |
| Space-complexity | _N + o(N)_                                             | _O(log N)_        | _O(log N)_   | _O(log N)_     | _O( max(log N, <u>max num of children a node has</u>) )_ |

(`node_num_to_index()` and `child_to_parent()` use [rank()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html#method.rank). `index_to_node_num()` and `parent_to_children()` use [select()](https://laysakura.github.io/succinct.rs/succinct_rs/succinct_bit_vector/struct.SuccinctBitVector.html#method.select)).

## Versions
Succinct.rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

### Rust Version Supports

Succinct.rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version
- Beta version
- Nightly build

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Roadmap

Succinct.rs has plan to provide these succinct data structures.

1. Succinct Bit Vector **(done)**
2. [LOUDS](https://dl.acm.org/citation.cfm?id=1398646) **(doing)**
    - Find out efficient API sets by applying LOUDS to [Trie](https://en.wikipedia.org/wiki/Trie) implementation.
3. [SuRF](http://www.pdl.cmu.edu/PDL-FTP/Storage/surf_sigmod18.pdf)

## Contributing

Any kind of pull requests are appreciated.

Currently, there are not many rules for contribution.
But at least your pull requests must pass Travis CI.

## License

Succinct.rs is dual licensed under the Apache 2.0 license and the MIT license.
