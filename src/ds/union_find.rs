use std::cmp::Ordering;

/// A Union-Find (Disjoint Set Union) data structure with path compression and
/// union by rank.
///
/// Supports `union`, `find`, and `connected` operations in amortized time
/// *O(α(n))*, where *α* is the inverse Ackermann function.
pub struct UnionFind {
  parent: Vec<usize>,
  rank: Vec<u8>,
  count: usize,
}

impl UnionFind {
  /// Create a new Union-Find structure with `n` elements (0 to n-1), each in
  /// its own set.
  ///
  /// Time complexity: `O(n)`.
  pub fn new(n: usize) -> Self {
    UnionFind {
      parent: (0..n).collect(),
      rank: vec![0; n],
      count: n,
    }
  }

  /// Find the representative (root) of the set containing `x`.
  ///
  /// Time complexity: *O(α(n))* amortized.
  pub fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
      self.parent[x] = self.find(self.parent[x]); // Path compression
    }
    self.parent[x]
  }

  /// Unite the sets containing `x` and `y`.
  ///
  /// Return `true` if the sets were separate and have been united, or `false`
  /// if they were already in the same set.
  ///
  /// Time complexity: *O(α(n))* amortized.
  pub fn union(&mut self, x: usize, y: usize) -> bool {
    let rx = self.find(x);
    let ry = self.find(y);

    if rx == ry {
      return false;
    }

    match self.rank[rx].cmp(&self.rank[ry]) {
      Ordering::Less => self.parent[rx] = ry,
      Ordering::Greater => self.parent[ry] = rx,
      Ordering::Equal => {
        self.parent[ry] = rx;
        self.rank[rx] += 1;
      }
    }

    self.count -= 1;
    true
  }

  /// Return `true` if `x` and `y` are in the same set, `false` otherwise.
  ///
  /// Time complexity: *O(α(n))* amortized.
  pub fn connected(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }

  /// Return the number of disjoint sets currently in the structure.
  ///
  /// Time complexity: `O(1)`.
  pub fn components(&self) -> usize {
    self.count
  }
}
