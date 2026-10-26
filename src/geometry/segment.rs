use crate::algebra::Zero;
use crate::geometry::Point;

/// A line segment between two points.
pub struct Segment<T> {
  pub start: Point<T>,
  pub end: Point<T>,
}

/// The intersection of two line segments.
pub enum Intersection<T> {
  /// The segments do not intersect.
  None,
  /// The segments intersect at a single point.
  Point(Point<T>),
  /// The segments overlap in a segment.
  Segment(Segment<T>),
}

impl<T> Segment<T> {
  /// Construct a segment between `start` and `end`.
  pub fn new(start: Point<T>, end: Point<T>) -> Self {
    Self { start, end }
  }

  /// Determine the intersection with another line segment.
  pub fn intersects(&self, other: &Segment<T>) -> Intersection<f64>
  where
    T: Copy + Into<f64>,
  {
    let p1 = Point::new(self.start.x.into(), self.start.y.into());
    let p2 = Point::new(self.end.x.into(), self.end.y.into());
    let q1 = Point::new(other.start.x.into(), other.start.y.into());
    let q2 = Point::new(other.end.x.into(), other.end.y.into());

    let dp = p2 - p1;
    let dq = q2 - q1;

    match (dp.norm_sq().is_zero(), dq.norm_sq().is_zero()) {
      (true, true) => {
        // Both segments are points
        if p1 == q1 {
          return Intersection::Point(p1);
        }
        return Intersection::None;
      }
      (true, false) => {
        // Self is a point
        if p1.in_bound(&q1, &q2) {
          return Intersection::Point(p1);
        }
        return Intersection::None;
      }
      (false, true) => {
        // Other is a point
        if q1.in_bound(&p1, &p2) {
          return Intersection::Point(q1);
        }
        return Intersection::None;
      }
      _ => {}
    }

    let dp_x_dq = dp.cross(dq);
    let p1q1 = q1 - p1;
    let p1q2 = q2 - p1;

    // Check if segments are NOT parallel: ∆p × ∆q ≠ 0
    if !dp_x_dq.is_zero() {
      let s = p1q1.cross(dq) / dp_x_dq;
      let t = p1q1.cross(dp) / dp_x_dq;

      // Are both s and t in the range [0, 1]?
      if between(s, 0.0, 1.0) && between(t, 0.0, 1.0) {
        // r = p1 + s * ∆p = q1 + t * ∆q
        return Intersection::Point(p1 + dp * s);
      }

      // Intersection outside of segments
      return Intersection::None;
    }

    // Segments are parallel.
    // Check if they are NOT collinear: ∆p × (q1 - p1) ≠ 0
    if !dp.cross(p1q1).is_zero() {
      return Intersection::None;
    }

    // Segments are collinear. Project into 1D and check for overlap.
    let dp2 = dp.dot(dp);
    let a1 = p1q1.dot(dp) / dp2; // alpha_1 from Popup L12
    let a2 = p1q2.dot(dp) / dp2; // alpha_2 from Popup L12

    // The set of intersection points (relative to p1-p2)
    // is the intersection [0,1] ∩ [min(a1,a2), max(a1,a2)]
    let lo = a1.min(a2).max(0.0);
    let hi = a1.max(a2).min(1.0);

    // Check for single intersection: lo == hi
    if (lo - hi).is_zero() {
      return Intersection::Point(p1 + dp * lo);
    }

    // Check for no intersection: lo > hi
    if lo > hi {
      return Intersection::None;
    }

    // Segments overlap in a segment from p1 + lo * ∆p to p1 + hi * ∆p
    let r1 = p1 + dp * lo;
    let r2 = p1 + dp * hi;
    Intersection::Segment(Segment::new(r1, r2))
  }

  /// Compute the minimum distance between two line segments.
  pub fn distance(&self, other: &Segment<T>) -> f64
  where
    T: Copy + Into<f64>,
  {
    match self.intersects(other) {
      Intersection::None => {
        let d1 = self.distance_to_point(&other.start);
        let d2 = self.distance_to_point(&other.end);
        let d3 = other.distance_to_point(&self.start);
        let d4 = other.distance_to_point(&self.end);
        d1.min(d2).min(d3).min(d4)
      }
      _ => 0.0,
    }
  }

  /// Compute the minimum distance between the segment and a point.
  pub fn distance_to_point(&self, p: &Point<T>) -> f64
  where
    T: Copy + Into<f64>,
  {
    let p = Point::new(p.x.into(), p.y.into());
    let q1 = Point::new(self.start.x.into(), self.start.y.into());
    let q2 = Point::new(self.end.x.into(), self.end.y.into());

    let dq = q2 - q1;
    if dq.norm_sq().is_zero() {
      return p.dist(q1);
    }

    let q1p = p - q1;
    let q2p = p - q2;

    // Is p between q1 and q2?
    if gt(q1p.dot(dq), 0.0) && lt(q2p.dot(dq), 0.0) {
      // Distance from p to line through q1 and q2
      q1p.cross(dq).abs() / dq.norm()
    } else {
      // Distance from p to the closer of q1 and q2
      q1p.norm().min(q2p.norm())
    }
  }
}

#[inline]
fn gt(x: f64, y: f64) -> bool {
  x > y || (x - y).is_zero()
}

#[inline]
fn lt(x: f64, y: f64) -> bool {
  x < y || (x - y).is_zero()
}

#[inline]
fn between(x: f64, min: f64, max: f64) -> bool {
  gt(x, min) && lt(x, max)
}
