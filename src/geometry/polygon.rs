use std::ops::{Add, Mul, Sub};

use crate::algebra::Zero;
use crate::geometry::{Orientation, Point, orient};

/// A (simple) polygon represented by a list of vertices in order.
pub struct Polygon<T> {
  vertices: Vec<Point<T>>,
}

/// The result of a point-in-polygon test.
pub enum Containment {
  Inside,
  Boundary,
  Outside,
}

impl<T> Polygon<T> {
  /// Create a new polygon from the given vertices.
  pub fn new(vertices: Vec<Point<T>>) -> Self {
    Self { vertices }
  }

  /// Return a reference to the vertices of the polygon.
  pub fn vertices(&self) -> &[Point<T>] {
    &self.vertices
  }
}

impl<T> Polygon<T>
where
  T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
  /// Calculate the signed area of the polygon. The sign indicates the
  /// orientation of the vertices: positive for counter-clockwise and negative
  /// for clockwise.
  ///
  /// Time complexity: *O(n)*.
  pub fn signed_area(&self) -> f64
  where
    T: Into<f64>,
  {
    let n = self.vertices.len();
    if n < 3 {
      return 0.0; // Degenerate
    }

    let mut sum = 0.0;
    for i in 0..n {
      let current = self.vertices[i];
      let next = self.vertices[(i + 1) % n]; // Wrap around

      sum += current.cross(next).into();
    }
    sum * 0.5
  }

  /// Calculate the area of the polygon.
  ///
  /// Time complexity: *O(n)*.
  pub fn area(&self) -> f64
  where
    T: Into<f64>,
  {
    self.signed_area().abs()
  }

  /// Determine whether `p` is inside, on the boundary of, or outside the
  /// polygon.
  ///
  /// Time complexity: *O(n)*.
  pub fn contains(&self, p: &Point<T>) -> Containment
  where
    T: Zero + PartialOrd,
  {
    let n = self.vertices.len();

    let mut winding = 0;
    for i in 0..n {
      let a = self.vertices[i];
      let b = self.vertices[(i + 1) % n];

      match orient(p, &a, &b) {
        Orientation::CounterClockwise => {
          if a.y <= p.y && b.y > p.y {
            winding += 1;
          }
        }
        Orientation::Collinear => {
          if p.in_bound(&a, &b) {
            return Containment::Boundary;
          }
        }
        Orientation::Clockwise => {
          if a.y > p.y && b.y <= p.y {
            winding -= 1;
          }
        }
      }
    }

    if winding != 0 {
      Containment::Inside
    } else {
      Containment::Outside
    }
  }
}
