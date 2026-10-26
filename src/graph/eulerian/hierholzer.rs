use crate::graph::Graph;

/// Find an Eulerian walk using Hierholzer's algorithm.
///
/// The returned vector contains the vertices of the walk in traversal order.
/// Consecutive vertices in the result correspond to edges in the input graph.
///
/// Returns `None` if the graph has no Eulerian walk or contains no edges.
///
/// An Eulerian walk uses every edge exactly once. The walk may be open
/// (different start and end vertices) or closed (an Eulerian circuit).
///
/// For a directed graph, an Eulerian walk requires all vertices to have equal
/// in- and out-degree except possibly one start vertex with one additional
/// outgoing edge and one end vertex with one additional incoming edge.
///
/// # Complexity
/// * Time complexity: *O(V + E)*.
/// * Auxiliary space: *O(V + E)*.
///
/// where *V* is the number of vertices and *E* is the number of edges.
pub fn hierholzer<E>(graph: &Graph<E>) -> Option<Vec<usize>> {
  let n = graph.vertex_count();
  let m = graph.edge_count();

  if m == 0 {
    return None; // No edges, no walk
  }

  let mut in_deg = vec![0; n];
  for u in 0..n {
    for edge in graph.edges(u) {
      in_deg[edge.to] += 1;
    }
  }

  // Find the start vertex and verify Eulerian conditions
  let mut start = None;
  let mut starts = 0;

  for (v, &d_in) in in_deg.iter().enumerate() {
    let d_out = graph.edges(v).len();
    let diff = d_out as isize - d_in;

    match diff {
      1 => {
        start = Some(v);
        starts += 1;
      }
      0 | -1 => {}
      _ => return None,
    }
  }

  if starts > 1 {
    return None;
  }

  // Eulerian circuit: start at any vertex with an outgoing edge
  let start = start.unwrap_or_else(|| (0..n).find(|&v| !graph.edges(v).is_empty()).unwrap());

  // Hierholzer's algorithm
  let mut next_edge = vec![0; n];
  let mut stack = vec![start];
  let mut walk = Vec::with_capacity(m + 1);

  while let Some(&u) = stack.last() {
    let edges = graph.edges(u);
    if next_edge[u] < edges.len() {
      // Traverse the next unvisited edge from u
      let edge = &edges[next_edge[u]];
      next_edge[u] += 1;
      stack.push(edge.to);
    } else {
      // No more unvisited outgoing edges, add to walk and backtrack
      walk.push(stack.pop().unwrap());
    }
  }

  // If we didn't traverse every edge, the graph has disconnected nonzero-degree
  // components and no Eulerian walk exists.
  if walk.len() != m + 1 {
    return None;
  }

  walk.reverse();
  Some(walk)
}
