use std::collections::VecDeque;

use crate::graph::Graph;
use crate::graph::flow::{ResidualData, ford_fulkerson};
use crate::graph::model::FlowData;

/// The amount of flow sent along a directed edge.
pub struct EdgeFlow {
  pub from: usize,
  pub to: usize,
  pub amount: u64,
}

/// Result of a maximum-flow computation.
pub struct MaxFlow {
  /// The value of the resulting flow.
  pub total_flow: u64,
  /// The positive flow on each original network edge.
  pub flows: Vec<EdgeFlow>,
}

/// Find an augmenting path in a residual network using breadth-first search.
///
/// Only residual edges with positive capacity are traversed. The returned
/// path contains `(vertex, edge_index)` pairs identifying the residual edges
/// along the path from `source` to `sink`.
///
/// Returns `None` if the sink is unreachable from the source in the residual
/// network.
pub fn find_augmenting_path_bfs(
  network: &Graph<ResidualData>,
  source: usize,
  sink: usize,
) -> Option<Vec<(usize, usize)>> {
  let n = network.vertex_count();

  let mut visited = vec![false; n];
  let mut parent = vec![None; n];
  let mut queue = VecDeque::new();

  visited[source] = true;
  queue.push_back(source);

  while let Some(u) = queue.pop_front() {
    for (edge_ix, edge) in network.edges(u).iter().enumerate() {
      if edge.data.capacity == 0 || visited[edge.to] {
        continue;
      }

      let v = edge.to;
      visited[v] = true;
      parent[v] = Some((u, edge_ix));

      if v == sink {
        let mut path = Vec::new();
        let mut current = sink;

        while current != source {
          let (u, edge_ix) = parent[current].unwrap();
          path.push((u, edge_ix));
          current = u;
        }

        path.reverse();
        return Some(path);
      }

      queue.push_back(v);
    }
  }

  None
}

/// Compute a maximum flow using the Edmonds–Karp algorithm.
///
/// Edmonds–Karp is the Ford–Fulkerson method with breadth-first search used
/// to select each augmenting path. The resulting flow is therefore a
/// maximum flow of the input network.
///
/// Returns the total flow value together with the positive flow on each
/// original network edge.
///
/// # Complexity
/// * Time complexity: *O(V * E²)*.
/// * Auxiliary space: *O(V + E)*.
///
/// where *V* is the number of vertices and *E* is the number of edges in
/// the residual network.
pub fn edmonds_karp(network: &Graph<FlowData>, source: usize, sink: usize) -> MaxFlow {
  let (residual, forward_edge_ids) =
    ford_fulkerson(network, source, sink, find_augmenting_path_bfs);

  // Construct output
  let mut flows = Vec::new();
  for (u, edge_ix) in forward_edge_ids {
    let forward = &residual.edges(u)[edge_ix];
    let reverse = &residual.edges(forward.to)[forward.data.rev_ix];
    let amount = reverse.data.capacity;
    if amount > 0 {
      flows.push(EdgeFlow {
        from: u,
        to: forward.to,
        amount,
      });
    }
  }

  let total_flow = flows
    .iter()
    .filter(|f| f.from == source)
    .map(|f| f.amount)
    .sum();

  MaxFlow { total_flow, flows }
}
