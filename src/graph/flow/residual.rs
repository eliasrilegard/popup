use crate::graph::Graph;
use crate::graph::model::FlowData;

/// Edge data used in a residual network.
///
/// `capacity` is the amount of additional flow that can currently be sent
/// along the edge. `cost_per` is the cost of sending one unit of flow along
/// the edge.
///
/// Each residual edge has a corresponding reverse edge. `rev_ix` stores the
/// index of that reverse edge in the adjacency list of `to`.
pub struct ResidualData {
  pub capacity: u64,
  pub cost_per: i64,
  pub rev_ix: usize,
}

/// Construct the residual network corresponding to `network`.
///
/// Each input edge produces a forward residual edge with the original
/// capacity and cost, and a reverse residual edge with zero capacity and
/// negated cost.
///
/// Returns the residual graph together with the locations of the forward
/// residual edges corresponding to the original edges. These locations can
/// be used to reconstruct the resulting flow after a flow algorithm has
/// finished.
pub fn build_residual_graph(
  network: &Graph<FlowData>,
) -> (Graph<ResidualData>, Vec<(usize, usize)>) {
  let n = network.vertex_count();

  let mut residual = Graph::new(n);
  let mut forward_edge_ids = Vec::new();

  for u in 0..n {
    for edge in network.edges(u) {
      let forward_index = residual.edges(u).len();
      let backward_index = residual.edges(edge.to).len();

      residual.add_edge(u, edge.to, ResidualData {
        capacity: edge.data.capacity,
        cost_per: edge.data.cost_per,
        rev_ix: backward_index,
      });
      residual.add_edge(edge.to, u, ResidualData {
        capacity: 0,
        cost_per: -edge.data.cost_per,
        rev_ix: forward_index,
      });

      forward_edge_ids.push((u, forward_index));
    }
  }

  (residual, forward_edge_ids)
}
