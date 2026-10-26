use std::collections::VecDeque;
use std::ops::Add;

use crate::graph::shortest_path::ShortestPaths;
use crate::graph::{Distance, Edge, Graph};

/// Compute shortest paths from `start` using the Bellman–Ford algorithm.
///
/// Each edge's data is interpreted as its traversal cost.
///
/// Unlike Dijkstra's algorithm, Bellman–Ford supports negative edge costs.
/// Vertices reachable from a reachable negative-weight cycle are marked
/// with [`Distance::NegativeCycle`].
///
/// Returns the computed distance and predecessor edge for every vertex.
/// Shortest paths can be reconstructed with [`ShortestPaths::path_to`].
///
/// # Complexity
/// * Time complexity: *O(V * E)* in the worst case.
/// * Auxiliary space: *O(V)*.
///
/// where *V* is the number of vertices and *E* is the number of edges.
pub fn bellman_ford<W>(graph: &Graph<W>, start: usize) -> ShortestPaths<W>
where
  W: Copy + Ord + Add<Output = W> + Default,
{
  bellman_ford_with(graph, start, |_dist, edge| Some(edge.data))
}

/// Compute shortest paths from `start` using a generalized Bellman–Ford
/// algorithm.
///
/// `edge_cost` computes the cost of traversing an edge given the current
/// distance. Returning `None` means that the edge cannot be traversed from
/// that distance.
///
/// Unlike Dijkstra's algorithm, Bellman–Ford supports negative edge costs.
/// Vertices whose shortest distances are affected by a reachable
/// negative-weight cycle are marked with [`Distance::NegativeCycle`].
///
/// The returned predecessor information identifies the edge used to reach
/// each vertex and can be used to reconstruct finite shortest paths with
/// [`ShortestPaths::path_to`].
///
/// # Complexity
/// * Time complexity: *O(V * E * (1 + T))* in the worst case.
/// * Auxiliary space: *O(V)*.
///
/// where *V* is the number of vertices, *E* is the number of edges, and *T* is
/// the time complexity of `edge_cost`.
pub fn bellman_ford_with<E, F, D>(
  graph: &Graph<E>,
  start: usize,
  mut edge_cost: F,
) -> ShortestPaths<D>
where
  D: Copy + Ord + Add<Output = D> + Default,
  F: FnMut(D, &Edge<E>) -> Option<D>,
{
  let n = graph.vertex_count();

  let mut distance = vec![Distance::Unreachable; n];
  let mut parent = vec![None; n];

  let zero = D::default();
  distance[start] = Distance::Finite(zero);

  // 1. Relax edges V - 1 times
  for _ in 0..n - 1 {
    let mut updated = false;

    for u in 0..n {
      let Distance::Finite(dist) = distance[u] else {
        continue;
      };

      for (edge_ix, edge) in graph.edges(u).iter().enumerate() {
        let Some(cost) = edge_cost(dist, edge) else {
          continue;
        };

        let candidate = dist + cost;
        if distance[edge.to].is_better(&candidate) {
          distance[edge.to] = Distance::Finite(candidate);
          parent[edge.to] = Some((u, edge_ix));
          updated = true;
        }
      }
    }

    // No distance was updated in this iteration, so we can stop early
    if !updated {
      break;
    }
  }

  // 2. Check for negative weight cycles
  let mut queue = VecDeque::new();
  for u in 0..n {
    let Distance::Finite(dist) = distance[u] else {
      continue;
    };

    for edge in graph.edges(u) {
      let Some(edge_cost) = edge_cost(dist, edge) else {
        continue;
      };
      let candidate = dist + edge_cost;

      if matches!(distance[edge.to], Distance::Finite(current) if candidate < current) {
        distance[edge.to] = Distance::NegativeCycle;
        parent[edge.to] = None;
        queue.push_back(edge.to);
      }
    }
  }

  // 3. BFS propagate negative cycle information
  while let Some(u) = queue.pop_front() {
    for edge in graph.edges(u) {
      if matches!(distance[edge.to], Distance::NegativeCycle) {
        continue;
      }

      distance[edge.to] = Distance::NegativeCycle;
      parent[edge.to] = None;
      queue.push_back(edge.to);
    }
  }

  ShortestPaths { distance, parent }
}
