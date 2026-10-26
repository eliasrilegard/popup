//! Computational geometry algorithms and data types.

mod closest_pair;
mod point;
mod polygon;
mod segment;

pub use closest_pair::closest_pair;
pub use point::Point;
use point::{Orientation, orient};
pub use polygon::{Containment, Polygon};
pub use segment::{Intersection, Segment};
