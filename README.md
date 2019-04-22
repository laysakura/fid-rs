# fid-rs

High performance FID (Fully Indexable Dictionary) library.

[Master API Docs](https://laysakura.github.io/fid-rs/fid_rs/)
|
[Released API Docs](https://docs.rs/crate/fid_rs)
|
[Benchmark Results](https://laysakura.github.io/fid-rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/fid-rs/blob/master/CHANGELOG.md)

[![Build Status](https://travis-ci.com/laysakura/fid-rs.svg?branch=master)](https://travis-ci.com/laysakura/fid-rs)
[![Crates.io](https://img.shields.io/crates/v/fid_rs.svg)](https://crates.io/crates/fid_rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/fid-rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/fid-rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/fid-rs/blob/master/LICENSE-APACHE)

## Quickstart

To use fid-rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
fid_rs = "0.1"
```

### Usage Overview

```rust
use fid_rs::Fid;

let fid = Fid::from("0100_1");  // Tips: Fid::from::<&str>() ignores '_'.

// Basic operations ---------------------
assert_eq!(fid.access(0), false);  // [0]1001; 0th bit is '0' (false)
assert_eq!(fid.access(1), true);   // 0[1]001; 1st bit is '1' (true)
assert_eq!(fid.access(4), true);   // 0100[1]; 4th bit is '1' (true)

assert_eq!(fid.rank(0), 0);  // [0]1001; Range [0, 0] has no '1'
assert_eq!(fid.rank(3), 1);  // [0100]1; Range [0, 3] has 1 '1'
assert_eq!(fid.rank(4), 2);  // [01001]; Range [0, 4] has 2 '1's

assert_eq!(fid.select(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '1's is i=0
assert_eq!(fid.select(1), Some(1)); // 0[1]001; Minimum i where range [0, i] has 1 '1's is i=1
assert_eq!(fid.select(2), Some(4)); // 0100[1]; Minimum i where range [0, i] has 2 '1's is i=4
assert_eq!(fid.select(3), None);    // There is no i where range [0, i] has 3 '1's

// rank0, select0 -----------------------
assert_eq!(fid.rank0(0), 1);  // [0]1001; Range [0, 0] has no '0'
assert_eq!(fid.rank0(3), 3);  // [0100]1; Range [0, 3] has 3 '0's
assert_eq!(fid.rank0(4), 3);  // [01001]; Range [0, 4] has 3 '0's

assert_eq!(fid.select0(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '0's is i=0
assert_eq!(fid.select0(1), Some(0)); // [0]1001; Minimum i where range [0, i] has 1 '0's is i=0
assert_eq!(fid.select0(2), Some(2)); // 01[0]01; Minimum i where range [0, i] has 2 '0's is i=2
assert_eq!(fid.select0(4), None);    // There is no i where range [0, i] has 4 '0's
```

### Constructors

```rust
use fid_rs::Fid;

// Most human-friendly way: Fid::from::<&str>()
let fid = Fid::from("0100_1");

// Complex construction in simple way: Fid::from::<&[bool]>()
let mut arr = [false; 5];
arr[1] = true;
arr[4] = true;
let fid = Fid::from(&arr[..]);
```

### Iterator

```rust
use fid_rs::Fid;

let fid = Fid::from("0100_1");

for bit in fid.iter() {
    println!("{}", bit);
}
// =>
// false
// true
// false
// false
// true
```

## Features

- **Arbitrary length support with minimum working memory**: fid-rs provides virtually _arbitrary size_ of FID. It is carefully designed to use as small memory space as possible.
- **Latest benchmark results are always accessible**: fid-rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/fid-rs/criterion/report/).

### Complexity

When the length of a `Fid` is _N_:

| Operation | Time-complexity | Space-complexity |
|-----------|-----------------|------------------|
| [Fid::from::<&str>()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#implementations) | _O(N)_ | _N + o(N)_ |
| [Fid::from::<&[bool]>()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#implementations) | _O(N)_ | _N + o(N)_ |
| [Fid::access()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.access) | _O(1)_ | _0_ |
| [Fid::rank()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.rank) | _O(1)_ | _O(log N)_ |
| [Fid::rank0()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.rank0) | _O(1)_ | _O(log N)_ |
| [Fid::select()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.select) | _O(log N)_ | _O(log N)_ |
| [Fid::select0()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.select0) | _O(log N)_ | _O(log N)_ |

(Actually, `select()`'s time-complexity can be _O(1)_ with complex implementation but fid-rs, like many other libraries, uses binary search of `rank()`'s result).

## Versions
fid-rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

## Rust Version Supports

fid-rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version
- Beta version
- Nightly build

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Contributing

Any kind of pull requests are appreciated.

### Guidelines

- `README.md` is generated from `$ cargo readme` command. Do not manually update `README.md` but edit `src/lib.rs` and then `$ cargo readme > README.md`.
- Travis CI automatically does the following commit & push to your pull-requests:
    - `$ cargo readme > README.md`
    - `$ cargo fmt --all`

## License

MIT OR Apache-2.0
