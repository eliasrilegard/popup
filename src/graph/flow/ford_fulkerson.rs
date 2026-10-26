use crate::graph::Graph;
use crate::graph::flow::{ResidualData, build_residual_graph};
use crate::graph::model::FlowData;

/// Compute a maximum flow by repeatedly augmenting along paths found by
/// `find_augmenting_path`.
///
/// The supplied function is responsible for finding an augmenting path in
/// the current residual network. The path is represented as a sequence of
/// `(vertex, edge_index)` pairs identifying residual edges.
///
/// The algorithm itself is the Ford–Fulkerson augmentation procedure:
/// repeatedly find an augmenting path, send the path bottleneck along it,
/// and update the corresponding forward and reverse residual capacities.
///
/// The choice of augmenting-path algorithm determines the properties of the
/// resulting algorithm. For example, using BFS yields the Edmonds–Karp
/// algorithm.
///
/// Returns the final residual network and the locations of the forward
/// residual edges corresponding to the original network edges.
///
/// # Complexity
/// * Time complexity: *O(F * T)*.
///
/// where *F* is the number of augmentation steps and *T* is the time complexity
/// of `find_augmenting_path`.
pub fn ford_fulkerson<F>(
  network: &Graph<FlowData>,
  source: usize,
  sink: usize,
  mut find_augmenting_path: F,
) -> (Graph<ResidualData>, Vec<(usize, usize)>)
where
  F: FnMut(&Graph<ResidualData>, usize, usize) -> Option<Vec<(usize, usize)>>,
{
  let (mut residual, forward_edge_ids) = build_residual_graph(network);

  while let Some(path) = find_augmenting_path(&residual, source, sink) {
    // Find bottleneck
    let bottleneck = path
      .iter()
      .map(|&(u, edge_ix)| residual.edges(u)[edge_ix].data.capacity)
      .min()
      .unwrap_or(0);

    // Update capacities
    for (u, edge_ix) in path {
      let (v, rev_ix) = {
        let edge = &residual.edges(u)[edge_ix];
        (edge.to, edge.data.rev_ix)
      };

      residual.edges_mut(u)[edge_ix].data.capacity -= bottleneck;
      residual.edges_mut(v)[rev_ix].data.capacity += bottleneck;
    }
  }

  (residual, forward_edge_ids)
}
