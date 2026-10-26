use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::algebra::Zero;

/// A point in 2D space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  /// Create a new point with coordinates `(x, y)`.
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T> Point<T>
where
  T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
  /// Compute the dot product of `self` and `other`.
  pub fn dot(self, other: Self) -> T {
    self.x * other.x + self.y * other.y
  }

  /// Compute the (2D) cross product of `self` and `other`.
  pub fn cross(self, other: Self) -> T {
    self.x * other.y - self.y * other.x
  }

  /// Compute the squared Euclidean norm of `self`.
  pub fn norm_sq(self) -> T {
    self.x * self.x + self.y * self.y
  }

  /// Compute the Euclidean norm of `self`.
  pub fn norm(self) -> f64
  where
    T: Into<f64>,
  {
    self.norm_sq().into().sqrt()
  }

  /// Compute the squared Euclidean distance between `self` and `other`.
  pub fn dist_sq(self, other: Self) -> T {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    dx * dx + dy * dy
  }

  /// Compute the Euclidean distance between `self` and `other`.
  pub fn dist(self, other: Self) -> f64
  where
    T: Into<f64>,
  {
    self.dist_sq(other).into().sqrt()
  }

  /// Compute the angle from `self` to `other`, in radians.
  ///
  /// The angle is measured counterclockwise from `self` to `other` and lies
  /// in the range `(-π, π]`.
  pub fn angle(self, other: Self) -> f64
  where
    T: Into<f64>,
  {
    let dot = self.dot(other).into();
    let cross = self.cross(other).into();
    cross.atan2(dot)
  }

  /// Determine whether `self` lies within the axis-aligned bounding box defined
  /// by points `a` and `b`.
  pub fn in_bound(&self, a: &Self, b: &Self) -> bool
  where
    T: PartialOrd,
  {
    let min_x = if a.x < b.x { a.x } else { b.x };
    let max_x = if a.x > b.x { a.x } else { b.x };
    let min_y = if a.y < b.y { a.y } else { b.y };
    let max_y = if a.y > b.y { a.y } else { b.y };

    self.x >= min_x && self.x <= max_x && self.y >= min_y && self.y <= max_y
  }
}

impl<T> fmt::Display for Point<T>
where
  T: fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

/// The orientation of three ordered points.
pub enum Orientation {
  /// The points turn counterclockwise.
  CounterClockwise,
  /// The points are collinear.
  Collinear,
  /// The points turn clockwise.
  Clockwise,
}

/// Determine the orientation of the ordered points `a`, `b`, and `c`.
pub fn orient<T>(a: &Point<T>, b: &Point<T>, c: &Point<T>) -> Orientation
where
  T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Zero,
{
  let cross = (*b - *a).cross(*c - *a);

  // Test for collinearity first to use trait-specific zero comparison
  if cross.is_zero() {
    return Orientation::Collinear;
  }

  // Not zero, determine sign
  match cross.partial_cmp(&T::ZERO).unwrap() {
    Ordering::Greater => Orientation::CounterClockwise,
    Ordering::Less => Orientation::Clockwise,
    _ => unreachable!(),
  }
}

// Blanket impls
// -------------

impl<T> Neg for Point<T>
where
  T: Neg<Output = T>,
{
  type Output = Point<T>;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl<T> Add for Point<T>
where
  T: Add<Output = T>,
{
  type Output = Point<T>;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T> Sub for Point<T>
where
  T: Sub<Output = T>,
{
  type Output = Point<T>;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T> Mul<T> for Point<T>
where
  T: Mul<Output = T> + Copy,
{
  type Output = Point<T>;

  fn mul(self, scalar: T) -> Self::Output {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

impl<T> Div<T> for Point<T>
where
  T: Div<Output = T> + Copy,
{
  type Output = Point<T>;

  fn div(self, scalar: T) -> Self::Output {
    Self {
      x: self.x / scalar,
      y: self.y / scalar,
    }
  }
}
