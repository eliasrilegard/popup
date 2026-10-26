//! Dynamic programming algorithms.

mod knapsack;
mod longest_increasing_subsequence;

pub use knapsack::{Item, knapsack};
pub use longest_increasing_subsequence::lis;
