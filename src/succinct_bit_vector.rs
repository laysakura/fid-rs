mod block;
mod blocks;
mod chunk;
mod chunks;
mod succinct_bit_vector;
mod succinct_bit_vector_builder;

use super::bit_string::BitString;
use super::internal_data_structure::popcount_table::PopcountTable;
use super::internal_data_structure::raw_bit_vector::RawBitVector;
use std::collections::HashSet;

/// Succinct bit vector.
///
/// This class can handle bit sequence of virtually **arbitrary length.**
///
/// In fact, _N_ (bit vector's length) is designed to be limited to: _N <= 2^64_.<br>
/// It should be enough for almost all usecases since a binary data of length of _2^64_ consumes _2^21 = 2,097,152_ TB (terabyte), which is hard to handle by state-of-the-art computer architecture.
///
/// # Examples
/// ```
/// extern crate succinct_rs;
///
/// use succinct_rs::{BitString, SuccinctBitVectorBuilder};
///
/// // Construction -------------------------
/// // `01001` built by `from_bit_string()`
/// let bv = SuccinctBitVectorBuilder::from_bit_string(BitString::new("0100_1")).build();  // Tips: BitString::new() ignores '_'.
///
/// // `01001` built by `from_length()` and `add_bit()`
/// let bv = SuccinctBitVectorBuilder::from_length(0)
///     .add_bit(false)
///     .add_bit(true)
///     .add_bit(false)
///     .add_bit(false)
///     .add_bit(true)
///     .build();
///
/// // Basic operations ---------------------
/// assert_eq!(bv.access(0), false);  // [0]1001; 0th bit is '0' (false)
/// assert_eq!(bv.access(1), true);   // 0[1]001; 1st bit is '1' (true)
/// assert_eq!(bv.access(4), true);   // 0100[1]; 4th bit is '1' (true)
///
/// assert_eq!(bv.rank(0), 0);  // [0]1001; Range [0, 0] has no '1'
/// assert_eq!(bv.rank(3), 1);  // [0100]1; Range [0, 3] has 1 '1'
/// assert_eq!(bv.rank(4), 2);  // [01001]; Range [0, 4] has 2 '1's
///
/// assert_eq!(bv.select(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '1's is i=0
/// assert_eq!(bv.select(1), Some(1)); // 0[1]001; Minimum i where range [0, i] has 1 '1's is i=1
/// assert_eq!(bv.select(2), Some(4)); // 0100[1]; Minimum i where range [0, i] has 2 '1's is i=4
/// assert_eq!(bv.select(3), None);    // There is no i where range [0, i] has 3 '1's
///
/// // rank0, select0 -----------------------
/// assert_eq!(bv.rank0(0), 1);  // [0]1001; Range [0, 0] has no '0'
/// assert_eq!(bv.rank0(3), 3);  // [0100]1; Range [0, 3] has 3 '0's
/// assert_eq!(bv.rank0(4), 3);  // [01001]; Range [0, 4] has 3 '0's
///
/// assert_eq!(bv.select0(0), Some(0)); // []01001; Minimum i where range [0, i] has 0 '0's is i=0
/// assert_eq!(bv.select0(1), Some(0)); // [0]1001; Minimum i where range [0, i] has 1 '0's is i=0
/// assert_eq!(bv.select0(2), Some(2)); // 01[0]01; Minimum i where range [0, i] has 2 '0's is i=2
/// assert_eq!(bv.select0(4), None);    // There is no i where range [0, i] has 4 '0's
/// ```
///
/// # Complexity
/// See [README](https://github.com/laysakura/succinct.rs/blob/master/README.md#succinct-bit-vector-complexity).
///
/// # Implementation detail
/// [access()](#method.access)'s implementation is trivial.
///
/// [select()](#method.select) just uses binary search of `rank()` results.
///
/// [rank()](#method.rank)'s implementation is standard but non-trivial.
/// So here explains implementation of _rank()_.
///
/// ## [rank()](#method.rank)'s implementation
/// Say you have the following bit vector.
///
/// ```text
/// 00001000 01000001 00000100 11000000 00100000 00000101 10100000 00010000 001 ; (N=67)
/// ```
///
/// Answer _rank(48)_ in _O(1)_ time-complexity and _o(N)_ space-complexity.
///
/// Naively, you can count the number of '1' from left to right.
/// You will find _rank(48) == 10_ but it took _O(N)_ time-complexity.
///
/// To reduce time-complexity to _O(1)_, you can use _memonization_ technique.<br>
/// Of course, you can memonize results of _rank(i)_ for every _i ([0, N-1])_.
///
/// ```text
/// Bit vector;   0  0  0  0  1  0  0  0  0  1  0  0  0  0  0  1  0  0  0  0  0  1  0  0  1  1  0  0  0  0  0  0  0  0  1  0  0  0  0  0  0  0  0  0  0  1  0  1  [1]  0  1  0  0  0  0  0  0  0  0  1  0  0  0  0  0  0  1 ; (N=67)
/// Memo rank(i); 0  0  0  0  1  1  1  1  1  2  2  2  2  2  2  3  3  3  3  3  3  4  4  4  5  6  6  6  6  6  6  6  6  6  7  7  7  7  7  7  7  7  7  7  7  8  8  9  10  10 11 11 11 11 11 11 11 11 11 12 12 12 12 12 12 12 13
/// ```
///
/// From this memo, you can answer _rank(48) == 10_ in constant time, although space-complexity for this memo is _O(N) > o(N)_.
///
/// To reduce space-complexity using memonization, we divide the bit vector into **Chunk** and **Block**.
///
/// ```text
/// Bit vector; 00001000 01000001 00000100 11000000 00100000 00000101 [1]0100000 00010000 001  ; (N=67)
/// Chunk;     |                  7                    |                12                  |  ; (size = (log N)^2 = 36)
/// Block;     |0 |1 |1  |2 |2 |3  |3 |4 |6  |6 |6  |7 |0 |0  |0 |2 |4    |4 |4  |5 |5 |5  |6| ; (size = (log N) / 2 = 3)
/// ```
///
/// - A **Chunk** has size of _(log N)^2_. Its value is _rank(<u>index of the last bit of the chunk</u>)_.
/// - A **Block** has size of _(log N) / 2_. A chunk has many blocks. Block's value is the number of '1's in _[<u>index of the first bit of the chunk the block belongs to</u>, <u>index of the last bit of the block</u>]_ (note that the value is reset to 0 at the first bit of a chunk).
///
/// Now you want to answer _rank(48)_. 48-th bit is in the 2nd chunk, and in the 5th block in the chunk.<br>
/// So the _rank(48)_ is at least:
///
///   _<u>7 (value of 1st chunk)</u> + <u>2 (value of 4th block in the 2nd chunk)</u>_
///
/// Then, focus on 3 bits in 5th block in the 2nd chunk; `[1]01`.<br>
/// As you can see, only 1 '1' is included up to 48-th bit (`101` has 2 '1's but 2nd '1' is 50-th bit, irrelevant to _rank(48)_).
///
/// Therefore, the _rank(48)_ is calculated as:
///
///   _<u>7 (value of 1st chunk)</u> + <u>2 (value of 4th block in the 2nd chunk)</u> + <u>1 ('1's in 5th block up to 48-th bit)</u>_
///
/// OK. That's all... Wait!<br>
/// _rank()_ must be in _O(1)_ time-complexity.
///
/// - _<u>7 (value of 1st chunk)</u>_: _O(1)_ if you store chunk value in array structure.
/// - _<u>2 (value of 4th block in the 2nd chunk)</u>_: Same as above.
/// - _<u>1 ('1's in 5th block up to 48-th bit)</u>_: **_O(<u>length of block</u>) = O(log N)_** !
///
/// Counting '1's in a block must also be _O(1)_, while using _o(N)_ space.<br>
/// We use **Table** for this purpose.
///
/// | Block content | Number of '1's in block |
/// |---------------|-------------------------|
/// | `000`         | 0                       |
/// | `001`         | 1                       |
/// | `010`         | 1                       |
/// | `011`         | 2                       |
/// | `100`         | 1                       |
/// | `101`         | 2                       |
/// | `110`         | 2                       |
/// | `111`         | 3                       |
///
/// This table is constructed in `build()`. So we can find the number of '1's in block in _O(1)_ time.<br>
/// Note that this table has _O(log N) = o(N)_ length.
///
/// In summary:
///
///   _rank() = (value of left chunk) + (value of left block) + (value of table keyed by inner block bits)_.
pub struct SuccinctBitVector {
    /// Raw data.
    rbv: RawBitVector,

    /// Total popcount of _[0, <u>last bit of the chunk</u>]_.
    ///
    /// Each chunk takes _2^64_ at max (when every bit is '1' for bit vector of length of _2^64_).
    /// A chunk has blocks.
    chunks: Chunks,

    /// Table to calculate inner-block `rank()` in _O(1)_.
    table: PopcountTable,
}

/// Builder of [SuccinctBitVector](struct.SuccinctBitVector.html).
pub struct SuccinctBitVectorBuilder {
    seed: SuccinctBitVectorSeed,
    bits_set: HashSet<u64>,
}

enum SuccinctBitVectorSeed {
    Length(u64),
    BitStr(BitString),
}

/// Collection of Chunk.
struct Chunks {
    chunks: Vec<Chunk>,
    chunks_cnt: u64,
}

/// Total popcount of _[0, <u>last bit of the chunk</u>]_ of a bit vector.
///
/// Each chunk takes _2^64_ at max (when every bit is '1' for SuccinctBitVector of length of _2^64_).
struct Chunk {
    value: u64, // popcount
    blocks: Blocks,

    #[allow(dead_code)]
    length: u16,
}

/// Collection of Block in a Chunk.
struct Blocks {
    blocks: Vec<Block>,
    blocks_cnt: u16,
}

/// Total popcount of _[_first bit of the chunk which the block belongs to_, _last bit of the block_]_ of a bit vector.
///
/// Each block takes (log 2^64)^2 = 64^2 = 2^16 at max (when every bit in a chunk is 1 for SuccinctBitVector of length of 2^64)
struct Block {
    value: u16, // popcount
    length: u8,
}
