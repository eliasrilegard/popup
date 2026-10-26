//! String algorithms.

mod aho_corasick;
mod kmp;
mod rolling_hash;
mod suffix_array;

pub use aho_corasick::AhoCorasick;
pub use kmp::kmp;
pub use rolling_hash::RollingHash;
pub use suffix_array::suffix_array;
