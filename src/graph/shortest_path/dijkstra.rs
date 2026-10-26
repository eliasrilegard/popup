use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use crate::graph::shortest_path::ShortestPaths;
use crate::graph::{Distance, Edge, Graph};

/// Compute shortest paths from `start` using Dijkstra's algorithm.
///
/// Each edge's data is interpreted as its traversal cost. All traversal
/// costs must be non-negative.
///
/// Returns the shortest distance and predecessor edge for every vertex.
/// Shortest paths can be reconstructed with [`ShortestPaths::path_to`].
///
/// # Complexity
/// * Time complexity: *O((V + E) log V)*.
/// * Auxiliary space: *O(V + E)*.
///
/// where *V* is the number of vertices and *E* is the number of edges.
pub fn dijkstra<W>(graph: &Graph<W>, start: usize) -> ShortestPaths<W>
where
  W: Copy + Ord + Add<Output = W> + Default,
{
  dijkstra_with(graph, start, |_dist, edge| Some(edge.data))
}

/// Compute shortest paths from `start` using a generalized form of
/// Dijkstra's algorithm.
///
/// `edge_cost` computes the cost of traversing an edge given the current
/// distance. Returning `None` prevents the edge from being traversed from
/// that distance.
///
/// The resulting edge weights must satisfy the assumptions required by
/// Dijkstra's algorithm. In particular, traversal must not invalidate a
/// vertex's finalized shortest distance; for ordinary static edge weights,
/// this is guaranteed when all edge costs are non-negative.
///
/// The returned predecessor information identifies the edge used to reach
/// each vertex, allowing shortest paths to be reconstructed with
/// [`ShortestPaths::path_to`].
///
/// # Complexity
/// * Time complexity: *O((V + E) log V + E * T)*.
/// * Auxiliary space: *O(V + E)*.
///
/// where *V* is the number of vertices, *E* is the number of edges, and *T* is
/// the time complexity of `edge_cost`.
pub fn dijkstra_with<E, F, D>(graph: &Graph<E>, start: usize, mut edge_cost: F) -> ShortestPaths<D>
where
  D: Copy + Ord + Add<Output = D> + Default,
  F: FnMut(D, &Edge<E>) -> Option<D>,
{
  let n = graph.vertex_count();

  let mut distance = vec![Distance::Unreachable; n];
  let mut parent = vec![None; n];
  let mut heap = BinaryHeap::new();

  let zero = D::default();
  distance[start] = Distance::Finite(zero);
  heap.push((Reverse(zero), start));

  while let Some((Reverse(dist), u)) = heap.pop() {
    // Each vertex is processed at most once
    if distance[u] != Distance::Finite(dist) {
      continue;
    }

    // Each edge is examined at most once
    for (edge_ix, edge) in graph.edges(u).iter().enumerate() {
      // O(E * T) time across all edges
      let Some(cost) = edge_cost(dist, edge) else {
        continue;
      };

      let candidate = dist + cost;
      if distance[edge.to].is_better(&candidate) {
        distance[edge.to] = Distance::Finite(candidate);
        parent[edge.to] = Some((u, edge_ix));

        // At most O(E) successful relaxations overall
        heap.push((Reverse(candidate), edge.to));
      }
    }
  }

  ShortestPaths { distance, parent }
}
