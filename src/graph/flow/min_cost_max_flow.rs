use crate::graph::flow::{ResidualData, ford_fulkerson};
use crate::graph::model::FlowData;
use crate::graph::{Graph, bellman_ford_with};

/// The amount of flow sent along a directed edge together with its cost.
pub struct EdgeFlow {
  pub from: usize,
  pub to: usize,
  pub amount: u64,
  pub cost: i64,
}

/// Result of a minimum-cost maximum-flow computation.
pub struct MinCostMaxFlow {
  /// The value of the maximum flow.
  pub total_flow: u64,
  /// The total cost of the maximum flow.
  pub total_cost: i64,
  /// The positive flow on each original network edge.
  pub flows: Vec<EdgeFlow>,
}

/// Finds a minimum-cost augmenting path in the residual network using
/// Bellman–Ford.
///
/// Residual edges with zero capacity are excluded from consideration, and
/// the cost of each traversed edge is its per-unit residual cost.
///
/// Returns the minimum-cost path from `source` to `sink`, if one exists.
fn find_augmenting_path_bf(
  network: &Graph<ResidualData>,
  source: usize,
  sink: usize,
) -> Option<Vec<(usize, usize)>> {
  let result = bellman_ford_with(network, source, |_, edge| {
    (edge.data.capacity > 0).then_some(edge.data.cost_per)
  });
  result.path_to(sink)
}

/// Compute a maximum flow of minimum total cost.
///
/// The algorithm repeatedly augments along a minimum-cost path in the
/// residual network. Bellman–Ford is used to find each augmenting path,
/// allowing residual edges with negative costs.
///
/// Returns the maximum flow together with its minimum total cost and the
/// positive flow on each original network edge.
///
/// # Complexity
/// * Time complexity: *O(F * V * E)*.
/// * Auxiliary space: *O(V + E)*.
///
/// where *F* is the number of augmentation steps, *V* is the number of
/// vertices, and *E* is the number of edges in the network.
pub fn min_cost_max_flow(network: &Graph<FlowData>, source: usize, sink: usize) -> MinCostMaxFlow {
  let (residual, forward_edge_ids) = ford_fulkerson(network, source, sink, find_augmenting_path_bf);

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
        cost: amount as i64 * forward.data.cost_per,
      });
    }
  }

  let total_flow = flows
    .iter()
    .filter(|f| f.from == source)
    .map(|f| f.amount)
    .sum();
  let total_cost = flows.iter().map(|f| f.cost).sum();

  MinCostMaxFlow {
    total_flow,
    total_cost,
    flows,
  }
}
