use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A complex number represented in Cartesian form.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Complex {
  pub re: f64,
  pub im: f64,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.im == 0.0 {
      write!(f, "{}", self.re)
    } else if self.re == 0.0 {
      write!(f, "{}i", self.im)
    } else if self.im < 0.0 {
      write!(f, "{} - {}i", self.re, -self.im)
    } else {
      write!(f, "{} + {}i", self.re, self.im)
    }
  }
}

impl Complex {
  pub const I: Self = Self { re: 0.0, im: 1.0 };
  pub const ONE: Self = Self { re: 1.0, im: 0.0 };
  pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

  /// Create a new complex number from real and imaginary parts.
  pub const fn new(re: f64, im: f64) -> Self {
    Self { re, im }
  }

  /// Create a complex number from polar coordinates (magnitude and angle).
  pub fn from_polar(r: f64, theta: f64) -> Self {
    Self {
      re: r * theta.cos(),
      im: r * theta.sin(),
    }
  }

  /// Return the complex conjugate, i.e. the reflection across the real axis.
  pub fn conj(self) -> Self {
    Self {
      re: self.re,
      im: -self.im,
    }
  }

  /// Return the squared absolute value (magnitude squared). This is more
  /// efficient than computing the absolute value directly and is often
  /// sufficient for comparisons.
  pub fn abs_sq(self) -> f64 {
    self.re * self.re + self.im * self.im
  }

  /// Return the absolute value (magnitude).
  pub fn abs(self) -> f64 {
    self.abs_sq().sqrt()
  }

  /// Return the multiplicative inverse (reciprocal).
  pub fn inv(self) -> Self {
    let norm_sq = self.abs_sq();
    Self {
      re: self.re / norm_sq,
      im: -self.im / norm_sq,
    }
  }

  /// Return the argument (angle from the real axis) in radians.
  pub fn arg(self) -> f64 {
    self.im.atan2(self.re)
  }
}

// Blanket impls
// -------------

impl Neg for Complex {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      re: -self.re,
      im: -self.im,
    }
  }
}

impl Add for Complex {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }
}

impl AddAssign for Complex {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Complex {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      re: self.re - other.re,
      im: self.im - other.im,
    }
  }
}

impl SubAssign for Complex {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Complex {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      re: self.re * rhs.re - self.im * rhs.im,
      im: self.re * rhs.im + self.im * rhs.re,
    }
  }
}

impl MulAssign for Complex {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Mul<f64> for Complex {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      re: self.re * rhs,
      im: self.im * rhs,
    }
  }
}

impl Div for Complex {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    let den = rhs.re * rhs.re + rhs.im * rhs.im;
    Self {
      re: (self.re * rhs.re + self.im * rhs.im) / den,
      im: (self.im * rhs.re - self.re * rhs.im) / den,
    }
  }
}

impl DivAssign for Complex {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Div<f64> for Complex {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      re: self.re / rhs,
      im: self.im / rhs,
    }
  }
}
