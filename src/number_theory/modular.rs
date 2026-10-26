use crate::number_theory::egcd;

/// Calculate the multiplicative inverse of `a` modulo `m`, if it exists.
///
/// The inverse is a value `x` satisfying `a * x ≡ 1 (mod m)`, and exists if
/// and only if `a` and `m` are coprime (i.e., `gcd(a, m) == 1`).
///
/// Return `None` if the inverse does not exist.
///
/// Time complexity: *O(log(max(a, m)))*.
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
  let (g, x, _) = egcd(a, m);
  (g == 1).then(|| x.rem_euclid(m))
}

/// Calculate `a * b` modulo `m`, without overflowing the intermediate product.
///
/// The result is in the range `0..m`.
///
/// Time complexity: *O(1)*.
pub fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
  (a as i128 * b as i128).rem_euclid(m as i128) as i64
}

/// Calculate `a / b` modulo `m`, if the modular inverse of `b` exists.
///
/// This is equivalent to `a * b⁻¹ (mod m)`, where `b⁻¹` is the multiplicative
/// inverse of `b` modulo `m`.
///
/// The result is in the range `0..m`. Return `None` if the modular inverse of
/// `b` does not exist, i.e., if `gcd(b, m) != 1`.
///
/// Time complexity: *O(log(max(b, m)))*.
pub fn mod_div(a: i64, b: i64, m: i64) -> Option<i64> {
  let inv = mod_inv(b, m)?;
  Some(mod_mul(a, inv, m))
}
