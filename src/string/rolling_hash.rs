/// Compute and query rolling hashes of substrings using polynomial hashing.
///
/// Hashes are computed using wrapping `u64` arithmetic, equivalent to
/// arithmetic modulo `2^64`.
pub struct RollingHash<const BASE: u64 = 257> {
  hash: Vec<u64>,
  power: Vec<u64>,
}

impl<const BASE: u64> RollingHash<BASE> {
  /// Construct a rolling hash for `bytes`.
  ///
  /// # Complexity
  /// * Time complexity: *O(n)*.
  /// * Auxiliary space: *O(n)*.
  pub fn new(bytes: impl AsRef<[u8]>) -> Self {
    let bytes = bytes.as_ref();
    let n = bytes.len();

    let mut hash = vec![0u64; n + 1];
    let mut power = vec![1u64; n + 1];

    for i in 0..n {
      hash[i + 1] = hash[i].wrapping_mul(BASE).wrapping_add(bytes[i] as u64);
      power[i + 1] = power[i].wrapping_mul(BASE);
    }

    Self { hash, power }
  }

  /// Get the hash of the substring `bytes[l..r]`.
  ///
  /// `l` is inclusive and `r` is exclusive.
  ///
  /// Time complexity: *O(1)*.
  pub fn get_hash(&self, l: usize, r: usize) -> u64 {
    assert!(l <= r && r < self.hash.len());
    self.hash[r].wrapping_sub(self.hash[l].wrapping_mul(self.power[r - l]))
  }
}
