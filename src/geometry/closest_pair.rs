use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use crate::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
  point: Point<f64>,
  idx: usize,
}

impl Eq for Node {}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .point
      .y
      .partial_cmp(&other.point.y)
      .unwrap()
      .then(self.point.x.partial_cmp(&other.point.x).unwrap())
      .then(self.idx.cmp(&other.idx))
  }
}

/// Find the closest pair of points in a slice of points. Return the indices
/// of the closest pair, or `None` if there are fewer than 2 points.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
pub fn closest_pair<T>(points: &[Point<T>]) -> Option<(usize, usize)>
where
  T: Copy + PartialOrd + Into<f64>,
{
  let n = points.len();
  if n < 2 {
    return None;
  }

  let points: Vec<Point<f64>> = points
    .iter()
    .map(|p| Point {
      x: p.x.into(),
      y: p.y.into(),
    })
    .collect();

  let mut order: Vec<_> = (0..n).collect();
  order.sort_unstable_by(|&i, &j| points[i].x.partial_cmp(&points[j].x).unwrap());

  let mut active: BTreeSet<Node> = BTreeSet::new();
  let mut queue: VecDeque<Node> = VecDeque::new();

  let mut dist_sq = f64::INFINITY;
  let mut best = (0, 1);

  for &i in &order {
    let dist = dist_sq.sqrt();
    let node = Node {
      point: points[i],
      idx: i,
    };

    // Remove points too far in x
    while let Some(front) = queue.front() {
      let dx = node.point.x - front.point.x;
      if dx > dist {
        active.remove(front);
        queue.pop_front();
      } else {
        break;
      }
    }

    // Query by y
    let lo = Node {
      point: Point::new(f64::NEG_INFINITY, node.point.y - dist),
      idx: 0,
    };
    let hi = Node {
      point: Point::new(f64::INFINITY, node.point.y + dist),
      idx: usize::MAX,
    };

    for q in active.range(lo..=hi) {
      let dx = node.point.x - q.point.x;
      let dy = node.point.y - q.point.y;
      let ds_sq = dx * dx + dy * dy;
      if ds_sq < dist_sq {
        dist_sq = ds_sq;
        best = (q.idx, i);
      }
    }

    // Insert current point
    active.insert(node);
    queue.push_back(node);
  }

  Some(best)
}
