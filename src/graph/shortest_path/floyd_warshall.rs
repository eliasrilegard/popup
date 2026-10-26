use std::ops::Add;

use crate::graph::{Distance, Graph};
use crate::linear_algebra::Matrix;

/// Compute shortest-path distances between all pairs of vertices using the
/// Floyd–Warshall algorithm.
///
/// Each edge's data is interpreted as its traversal cost. Multiple edges
/// between the same pair of vertices are supported; only the minimum edge
/// cost is considered.
///
/// Negative edge weights are supported. If a pair of vertices is connected
/// through a reachable negative-weight cycle, its distance is reported as
/// [`Distance::NegativeCycle`], since no finite shortest distance exists.
///
/// Returns [`Distance::Finite`] for pairs with a finite shortest path and
/// [`Distance::Unreachable`] for pairs with no path between them.
///
/// # Complexity
/// * Time complexity: *O(V³)*.
/// * Auxiliary space: *O(V²)*.
///
/// where *V* is the number of vertices in the graph.
pub fn floyd_warshall<W>(graph: &Graph<W>) -> Matrix<Distance<W>>
where
  W: Copy + Ord + Add<Output = W> + Default,
{
  let n = graph.vertex_count();

  // Matrix setup
  let mut distances = Matrix::new(n, n, vec![Distance::Unreachable; n * n]);
  for v in 0..n {
    for edge in graph.edges(v) {
      // Take minimum of multiple edges between the same pair of vertices
      if distances[(v, edge.to)].is_better(&edge.data) {
        distances[(v, edge.to)] = Distance::Finite(edge.data);
      }
    }
  }
  let zero = W::default();
  for v in 0..n {
    distances[(v, v)] = Distance::Finite(zero);
  }

  // Floyd–Warshall relaxation
  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        let (Distance::Finite(dist_ik), Distance::Finite(dist_kj)) =
          (distances[(i, k)], distances[(k, j)])
        else {
          continue;
        };

        let candidate = dist_ik + dist_kj;
        if distances[(i, j)].is_better(&candidate) {
          distances[(i, j)] = Distance::Finite(candidate);
        }
      }
    }
  }

  // Detect negative cycles: v is part of a negative cycle iff dist(v, v) < 0
  let negative_cycle_vertices: Vec<usize> = (0..n)
    .filter(|&v| matches!(distances[(v, v)], Distance::Finite(d) if d < zero))
    .collect();

  // If i → k is reachable and k → j is reachable, then i → j has distance −∞
  for k in negative_cycle_vertices {
    for i in 0..n {
      if matches!(distances[(i, k)], Distance::Unreachable) {
        continue;
      }

      for j in 0..n {
        if matches!(distances[(k, j)], Distance::Unreachable) {
          continue;
        }

        distances[(i, j)] = Distance::NegativeCycle;
      }
    }
  }

  distances
}
