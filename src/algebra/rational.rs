use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::number_theory::gcd;

/// A rational number represented as a reduced fraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rational {
  num: i64,
  den: i64,
}

impl fmt::Display for Rational {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}/{}", self.num, self.den)
  }
}

impl Rational {
  pub const ONE: Self = Self { num: 1, den: 1 };
  pub const ZERO: Self = Self { num: 0, den: 1 };

  /// Create a new rational number from a numerator and denominator. The
  /// fraction is automatically reduced to its simplest form.
  pub fn new(num: i64, den: i64) -> Self {
    assert!(den != 0, "Denominator cannot be zero");

    Self { num, den }.reduce()
  }

  fn reduce(mut self) -> Self {
    let g = gcd(self.num, self.den);
    self.num /= g;
    self.den /= g;

    if self.den < 0 {
      self.num = -self.num;
      self.den = -self.den;
    }

    self
  }

  /// Return the numerator.
  pub const fn num(&self) -> i64 {
    self.num
  }

  /// Return the denominator.
  pub const fn den(&self) -> i64 {
    self.den
  }
}

impl PartialOrd for Rational {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Rational {
  fn cmp(&self, other: &Self) -> Ordering {
    // Compare cross products: a/b < c/d if and only if a*d < c*b
    let ad = self.num as i128 * other.den as i128;
    let cb = other.num as i128 * self.den as i128;
    ad.cmp(&cb)
  }
}

impl Neg for Rational {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Rational::new(-self.num, self.den)
  }
}

impl Add for Rational {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let g = gcd(self.den, rhs.den);

    let den = self.den / g * rhs.den;
    let num1 = self.num * (rhs.den / g);
    let num2 = rhs.num * (self.den / g);

    Rational::new(num1 + num2, den)
  }
}

impl AddAssign for Rational {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Rational {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    self + (-rhs)
  }
}

impl SubAssign for Rational {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Rational {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let g1 = gcd(self.num, rhs.den);
    let g2 = gcd(rhs.num, self.den);

    Rational::new(
      (self.num / g1) * (rhs.num / g2),
      (self.den / g2) * (rhs.den / g1),
    )
  }
}

impl MulAssign for Rational {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Div for Rational {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    assert!(rhs.num != 0, "Cannot divide by zero");
    self * Rational::new(rhs.den, rhs.num)
  }
}

impl DivAssign for Rational {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}
