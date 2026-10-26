use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::algebra::{Complex, Rational};

pub trait Zero {
  const ZERO: Self;
  fn is_zero(&self) -> bool;
}

pub trait One {
  const ONE: Self;
}

pub trait Field:
  Zero
  + One
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
  + Neg<Output = Self>
  + Sized
{
}

macro_rules! impl_zero_one {
  [$($t:ty),*] => {
    $(
      impl Zero for $t {
        const ZERO: Self = 0 as $t;

        fn is_zero(&self) -> bool {
          *self == Self::ZERO
        }
      }

      impl One for $t {
        const ONE: Self = 1 as $t;
      }
    )*
  };
}

impl_zero_one![
  i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
];

const EPS: f64 = 1e-9;

impl Zero for f64 {
  const ZERO: Self = 0.0;

  fn is_zero(&self) -> bool {
    self.abs() < EPS
  }
}

impl One for f64 {
  const ONE: Self = 1.0;
}

impl Zero for f32 {
  const ZERO: Self = 0.0;

  fn is_zero(&self) -> bool {
    self.abs() < EPS as f32
  }
}

impl One for f32 {
  const ONE: Self = 1.0;
}

impl Zero for Rational {
  const ZERO: Self = Rational::ZERO;

  fn is_zero(&self) -> bool {
    self.num() == 0
  }
}

impl One for Rational {
  const ONE: Self = Rational::ONE;
}

impl Zero for Complex {
  const ZERO: Self = Complex::ZERO;

  fn is_zero(&self) -> bool {
    self.re.abs() < EPS && self.im.abs() < EPS
  }
}

impl One for Complex {
  const ONE: Self = Complex::ONE;
}

impl Field for f64 {}
impl Field for Rational {}
impl Field for Complex {}
