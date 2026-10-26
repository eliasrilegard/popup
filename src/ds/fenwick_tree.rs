use std::ops::{AddAssign, Sub};

/// A Fenwick Tree (Binary Indexed Tree) for efficient prefix sums and updates.
pub struct FenwickTree<T> {
  tree: Vec<T>,
  zero: T,
}

impl<T> FenwickTree<T>
where
  T: Copy + AddAssign + Sub<Output = T>,
{
  /// Create a new Fenwick Tree with `n` elements, initialized to `zero` (the
  /// additive identity).
  pub fn new(n: usize, zero: T) -> Self {
    Self {
      tree: vec![zero; n + 1],
      zero,
    }
  }

  /// Add `value` to the element at `index` (0-indexed).
  ///
  /// Time complexity: *O(log n)*.
  pub fn add(&mut self, mut index: usize, value: T) {
    index += 1; // The tree is 1-indexed

    while index < self.tree.len() {
      self.tree[index] += value;
      index += lsb(index);
    }
  }

  /// Return the sum of elements in the range `[0,end)`.
  ///
  /// Time complexity: *O(log n)*.
  pub fn prefix_sum(&self, mut end: usize) -> T {
    let mut result = self.zero;

    while end > 0 {
      result += self.tree[end];
      end -= lsb(end);
    }

    result
  }

  /// Return the sum of elements in the range `[left,right)`.
  ///
  /// Time complexity: *O(log n)*.
  pub fn range_sum(&self, left: usize, right: usize) -> T {
    self.prefix_sum(right) - self.prefix_sum(left)
  }
}

/// Return the Least Significant Bit of `n`, which is the largest power of 2
/// that divides `n`.
#[inline]
fn lsb(n: usize) -> usize {
  n & (!n + 1)
}
