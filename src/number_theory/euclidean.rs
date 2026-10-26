use std::ops::{Div, Mul, Rem, Sub};

use crate::algebra::{One, Zero};

/// Calculate the greatest common divisor of `a` and `b` and the Bézout
/// coefficients `x` and `y` satisfying `a * x + b * y = gcd(a, b)`.
///
/// Time complexity: *O(log(max(a, b)))*.
#[doc(alias = "extended_euclidean")]
pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
  T: Copy + Zero + One + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
  if b.is_zero() {
    (a, T::ONE, T::ZERO)
  } else {
    let (g, x1, y1) = egcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
  }
}

/// Calculate the greatest common divisor of `a` and `b` using the Euclidean
/// Algorithm.
///
/// Time complexity: *O(log(max(a, b)))*.
pub fn gcd(a: i64, b: i64) -> i64 {
  egcd(a, b).0
}

/// Calculate the least common multiple of `a` and `b`.
///
/// Time complexity: *O(log(max(a, b)))*.
pub fn lcm(a: i64, b: i64) -> i64 {
  a / gcd(a, b) * b
}
