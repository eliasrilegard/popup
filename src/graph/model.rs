/// A directed edge in a graph.
pub struct Edge<E> {
  /// The destination vertex of the edge.
  pub to: usize,
  /// The data associated with the edge.
  pub data: E,
}

/// A directed graph represented as an adjacency list.
pub struct Graph<E> {
  adj: Vec<Vec<Edge<E>>>,
}

impl<E> Graph<E> {
  /// Create a new graph with `n` vertices and no edges.
  ///
  /// Vertices are numbered from `0` to `n - 1`.
  pub fn new(n: usize) -> Self {
    Graph {
      adj: (0..n).map(|_| Vec::new()).collect(),
    }
  }

  /// Add a directed edge from `from` to `to` with the given `data`.
  pub fn add_edge(&mut self, from: usize, to: usize, data: E) {
    self.adj[from].push(Edge { to, data });
  }

  /// Add two directed edges between `from` and `to`, one in each direction.
  ///
  /// The same edge data is stored on both edges, requiring `E` to be [`Clone`].
  pub fn add_undirected_edge(&mut self, from: usize, to: usize, data: E)
  where
    E: Clone,
  {
    self.add_edge(from, to, data.clone());
    self.add_edge(to, from, data);
  }

  /// Return the outgoing edges from `from`.
  ///
  /// Edges are returned in the order in which they were added.
  pub fn edges(&self, from: usize) -> &[Edge<E>] {
    &self.adj[from]
  }

  /// Return a mutable slice containing the outgoing edges from `from`.
  ///
  /// Edges are returned in the order in which they were added.
  pub fn edges_mut(&mut self, from: usize) -> &mut [Edge<E>] {
    &mut self.adj[from]
  }

  /// Return the number of vertices in the graph.
  pub fn vertex_count(&self) -> usize {
    self.adj.len()
  }

  /// Return the number of directed edges in the graph.
  ///
  /// For an undirected edge added with [`Graph::add_undirected_edge`], both
  /// directed edges are counted.
  pub fn edge_count(&self) -> usize {
    self.adj.iter().map(Vec::len).sum()
  }
}

/// Data associated with an edge in a flow network.
pub struct FlowData {
  /// The maximum amount of flow that may be sent along the edge.
  pub capacity: u64,
  /// The cost incurred per unit of flow sent along the edge.
  pub cost_per: i64,
}

impl Graph<FlowData> {
  /// Add a directed edge with the given capacity and zero cost.
  pub fn add_flow_edge(&mut self, from: usize, to: usize, capacity: u64) {
    self.add_edge(from, to, FlowData {
      capacity,
      cost_per: 0,
    });
  }

  /// Add a directed edge with the given capacity and per-unit cost.
  pub fn add_flow_edge_with_cost(&mut self, from: usize, to: usize, capacity: u64, cost_per: i64) {
    self.add_edge(from, to, FlowData { capacity, cost_per });
  }
}
