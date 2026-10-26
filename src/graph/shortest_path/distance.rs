/// The shortest-path status of a vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance<D> {
  /// The vertex has a finite shortest-path distance.
  Finite(D),

  /// The vertex is unreachable from the source.
  Unreachable,

  /// The vertex is reachable from a negative cycle.
  ///
  /// No finite shortest-path distance exists for such a vertex.
  NegativeCycle,
}

impl<D: PartialOrd> Distance<D> {
  /// Return whether `candidate` improves this distance.
  ///
  /// An unreachable vertex is always improved by a finite candidate, while a
  /// vertex already affected by a negative cycle cannot be improved.
  pub fn is_better(&self, candidate: &D) -> bool {
    match self {
      Distance::Unreachable => true,
      Distance::Finite(current) => candidate < current,
      Distance::NegativeCycle => false,
    }
  }
}

/// Shortest-path information produced by a single-source shortest-path
/// algorithm.
pub struct ShortestPaths<D> {
  /// The shortest-path status or distance of each vertex.
  pub distance: Vec<Distance<D>>,

  /// The predecessor edge used to obtain each finite shortest path.
  ///
  /// Each entry is `(vertex, edge_index)`, identifying an edge in that
  /// vertex's adjacency list.
  pub parent: Vec<Option<(usize, usize)>>,
}

impl<D> ShortestPaths<D> {
  /// Reconstruct a shortest path to `target`.
  ///
  /// The returned path contains `(vertex, edge_index)` pairs identifying the
  /// edges traversed from the source to `target`. The source vertex itself is
  /// not included as a separate element.
  ///
  /// Returns `None` if `target` is unreachable or is affected by a negative
  /// cycle.
  pub fn path_to(&self, target: usize) -> Option<Vec<(usize, usize)>> {
    if !matches!(self.distance[target], Distance::Finite(_)) {
      return None;
    }

    let mut path = Vec::new();
    let mut current = target;

    while let Some((node, edge_ix)) = self.parent[current] {
      path.push((node, edge_ix));
      current = node;
    }

    path.reverse();

    Some(path)
  }
}
