use std::collections::VecDeque;

use crate::graph::Graph;
use crate::graph::flow::{find_augmenting_path_bfs, ford_fulkerson};
use crate::graph::model::FlowData;

/// Compute a minimum `s`–`t` cut using the max-flow/min-cut theorem.
///
/// The returned vertices are the vertices reachable from `s` in the final
/// residual network after computing a maximum flow. Thus the returned set
/// describes the source side of a minimum cut: vertices in the returned set
/// belong to the source side, while all other vertices belong to the sink
/// side.
///
/// Returns the source-side vertex set of a minimum cut.
pub fn min_cut(graph: &Graph<FlowData>, s: usize, t: usize) -> Vec<usize> {
  // Run max flow (Edmonds–Karp)
  let (residual, _) = ford_fulkerson(graph, s, t, find_augmenting_path_bfs);

  // Find vertices reachable from s in the residual graph (BFS)
  let n = residual.vertex_count();
  let mut reachable = vec![false; n];
  let mut queue = VecDeque::new();

  reachable[s] = true;
  queue.push_back(s);

  while let Some(u) = queue.pop_front() {
    for edge in residual.edges(u) {
      if edge.data.capacity > 0 && !reachable[edge.to] {
        reachable[edge.to] = true;
        queue.push_back(edge.to);
      }
    }
  }

  // Apply the Max-flow Min-cut theorem. Done!
  reachable
    .into_iter()
    .enumerate()
    .filter_map(|(v, possible)| possible.then_some(v))
    .collect()
}
