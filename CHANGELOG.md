# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.6.0] - 2019-04-10

### Added
- `succinct_rs::SuccinctBitVectorBuilder::add_bit()`

## [v0.5.0] - 2019-04-10
Renamed _BitVector_ to _SuccinctBitVector_.

### Added
- `succinct_rs::SuccinctBitVector`
- `succinct_rs::SuccinctBitVectorBuilder`

### Removed
- `succinct_rs::BitVector`
- `succinct_rs::BitVectorBuilder`

## [v0.4.0] - 2019-04-09

### Added
- Experimentally adds `succinct_rs::{Louds, LoudsBuilder, LoudsIndex, LoudsNodeNum}`. It is highly possible to break APIs for LOUDS family while implementing Trie as a usecase using these structs.

## [v0.3.0] - 2019-04-08

### Added
- `succinct_rs::BitVector::rank0()`
- `succinct_rs::BitVector::select0()`

## [v0.2.0] - 2019-04-07

### Added
- `succinct_rs::BitString`
- `succinct_rs::BitVectorBuilder::from_bit_string()`

### Removed
- `succinct_rs::BitVectorString`
- `succinct_rs::BitVectorBuilder::from_str()`

## [v0.1.1] - 2019-04-07

### Fixed
- Adds `readme = "README.md"` in `Cargo.toml` in order to display README contents in crates.io.

## [v0.1.0] - 2019-04-07

### Added
- `succinct_rs::BitVector` and its builders: `succinct_rs::BitVectorBuilder` and `succinct_rs::BitVectorString`.

[Unreleased]: https://github.com/laysakura/succinct.rs/compare/v0.6.0...HEAD
[v0.6.0]: https://github.com/laysakura/succinct.rs/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/laysakura/succinct.rs/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/laysakura/succinct.rs/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/laysakura/succinct.rs/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/laysakura/succinct.rs/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/laysakura/succinct.rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/laysakura/succinct.rs/compare/3d425b4...v0.1.0
