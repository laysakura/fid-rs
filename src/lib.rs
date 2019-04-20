//! # Succinct.rs
//!
//! Succinct.rs is a library to provide succinct data structures with _simple API_ and _high performance_.
//!
//! See [README](https://github.com/laysakura/succinct.rs/blob/master/README.md) for more about usage and features.

pub use bit_string::BitString;
pub use louds::{Louds, LoudsBuilder, LoudsIndex, LoudsNodeNum};
pub use fid::{Fid, FidBuilder};

pub mod bit_string;
mod internal_data_structure;
pub mod louds;
pub mod fid;
