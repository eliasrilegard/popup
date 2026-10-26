use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use crate::graph::Graph;

/// A minimum spanning tree produced by a spanning-tree algorithm.
pub struct MinimumSpanningTree<W> {
  /// The edges belonging to the spanning tree, represented as
  /// `(from, to, weight)` tuples.
  pub edges: Vec<(usize, usize, W)>,

  /// The sum of the weights of all edges in the spanning tree.
  pub total_weight: W,
}

/// Compute a minimum spanning tree using Prim's algorithm.
///
/// The graph is treated as an undirected graph. In particular, edges are
/// considered in the direction in which they are stored, so an undirected
/// edge should normally be represented by two directed edges using
/// [`Graph::add_undirected_edge`].
///
/// Returns `None` if the graph is disconnected and therefore has no spanning
/// tree.
///
/// # Complexity
/// * Time complexity: *O(E log V)*
/// * Auxiliary space: *O(V + E)* including the graph and priority queue.
///
/// where *V* is the number of vertices and *E* is the number of edges.
pub fn prim<W>(graph: &Graph<W>) -> Option<MinimumSpanningTree<W>>
where
  W: Copy + Ord + Add<Output = W> + Default,
{
  let n = graph.vertex_count();

  let mut visited = vec![false; n];
  let mut heap = BinaryHeap::new();
  let mut edges = Vec::new();

  let start = 0; // Arbitrary
  visited[start] = true;

  for edge in graph.edges(start) {
    heap.push((Reverse(edge.data), start, edge.to));
  }

  while let Some((Reverse(weight), u, v)) = heap.pop() {
    // At most O(E) heap removals, each O(log E)
    if visited[v] {
      continue;
    }

    visited[v] = true;
    edges.push((u, v, weight));

    // Each edge is visited at most once
    for edge in graph.edges(v) {
      if !visited[edge.to] {
        heap.push((Reverse(edge.data), v, edge.to));
      }
    }
  }

  // Connectness test
  if edges.len() != n - 1 {
    return None;
  }

  let total_weight = edges
    .iter()
    .map(|edge| edge.2)
    .reduce(|a, b| a + b)
    .unwrap_or_default();

  Some(MinimumSpanningTree {
    edges,
    total_weight,
  })
}
