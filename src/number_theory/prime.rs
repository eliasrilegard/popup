use crate::ds::BitVec;

/// Store the primality of all integers up to and including a given limit,
/// computed using the Sieve of Eratosthenes algorithm.
pub struct Eratosthenes {
  sieve: BitVec,
}

impl Eratosthenes {
  /// Compute the prime numbers up to and including `n`.
  ///
  /// # Complexity
  /// * Time complexity: *O(n log log n)*.
  /// * Auxiliary space: *O(n)* bits.
  pub fn new(n: usize) -> Self {
    let mut is_prime = BitVec::ones(n + 1);
    is_prime.unset(0);
    if n >= 1 {
      is_prime.unset(1);
    }

    for i in (2..).take_while(|&i| i <= n / i) {
      if is_prime.get(i) {
        for j in (i * i..=n).step_by(i) {
          is_prime.unset(j);
        }
      }
    }

    Self { sieve: is_prime }
  }

  /// Return the number of primes up to and including the sieve's limit.
  ///
  /// Time complexity: *O(n)*, where *n* is the sieve's limit.
  pub fn count(&self) -> usize {
    self.sieve.count_ones()
  }

  /// Return whether `n` is prime.
  ///
  /// Time complexity: *O(1)*.
  pub fn is_prime(&self, n: usize) -> bool {
    self.sieve.get(n)
  }
}
