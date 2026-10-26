use std::collections::VecDeque;

pub struct TrieNode {
  /// Next state for each byte.
  transition: [Option<usize>; 256],
  /// Failure link for the Aho-Corasick automaton.
  failure: usize,
  /// Indices of patterns ending at this node.
  output: Vec<usize>,
}

impl TrieNode {
  fn new() -> Self {
    Self {
      transition: [None; 256],
      failure: 0,
      output: Vec::new(),
    }
  }
}

const ROOT: usize = 0;

/// Aho-Corasick automaton for finding multiple patterns in a byte sequence.
///
/// The automaton is constructed from a collection of patterns and can then be
/// used to find all occurrences of every pattern in a text. Patterns may
/// overlap.
///
/// Pattern and match positions are reported as byte offsets.
pub struct AhoCorasick {
  nodes: Vec<TrieNode>,
  pattern_lengths: Vec<usize>,
}

impl AhoCorasick {
  /// Create a new Aho-Corasick automaton from the given patterns.
  ///
  /// Each pattern is assigned an index according to its position in `patterns`.
  /// These indices correspond to the ouput of [`Self::find_all`].
  ///
  /// # Complexity
  /// * Time complexity: *O(P + B)*.
  /// * Auxiliary space: *O(P + B)*.
  ///
  /// where *P* is the total length of all patterns and *B* is the total number
  /// of bytes stored in the transition tables. With a fixed-size transition
  /// representation, *B = 256N*, where *N* is the number of trie nodes.
  pub fn new<I, P>(patterns: I) -> Self
  where
    I: IntoIterator<Item = P>,
    P: AsRef<[u8]>,
  {
    let mut nodes = vec![TrieNode::new()];
    let mut pattern_lengths = Vec::new();

    // 1. Build trie
    for (index, pattern) in patterns.into_iter().enumerate() {
      let pattern = pattern.as_ref();
      pattern_lengths.push(pattern.len());

      let mut node = ROOT;

      for &byte in pattern {
        let b = byte as usize;

        let next = match nodes[node].transition[b] {
          Some(next) => next,
          None => {
            let next = nodes.len();
            nodes.push(TrieNode::new());
            nodes[node].transition[b] = Some(next);
            next
          }
        };

        node = next;
      }

      nodes[node].output.push(index);
    }

    // 2. Set up failure links
    let mut queue = VecDeque::new();
    for byte in 0..256 {
      if let Some(next) = nodes[ROOT].transition[byte] {
        nodes[next].failure = ROOT;
        queue.push_back(next);
      }
    }

    while let Some(node) = queue.pop_front() {
      for byte in 0..256 {
        let Some(child) = nodes[node].transition[byte] else {
          continue;
        };

        // Find child's failure destination
        let mut fail = nodes[node].failure;
        while fail != ROOT && nodes[fail].transition[byte].is_none() {
          fail = nodes[fail].failure;
        }
        nodes[child].failure = nodes[fail].transition[byte].unwrap_or(ROOT);

        // Merge output
        let inherited = nodes[nodes[child].failure].output.clone();
        nodes[child].output.extend(inherited);

        queue.push_back(child);
      }
    }

    Self {
      nodes,
      pattern_lengths,
    }
  }

  /// Find all occurrences of every pattern in `text`.
  ///
  /// Return a vector of byte offsets for each pattern, in the same order as
  /// the patterns were provided to [`Self::new`]. Overlapping occurences are
  /// included.
  ///
  /// # Complexity
  /// * Time complexity: *O(T + K)*.
  /// * Auxiliary space: *O(K + M)*.
  ///
  /// where *T* is the length of `text`, *K* is the total number of matches
  /// found, and *M* is the number of patterns.
  pub fn find_all(&self, text: impl AsRef<[u8]>) -> Vec<Vec<usize>> {
    let text = text.as_ref();

    let mut matches = vec![Vec::new(); self.pattern_lengths.len()];
    let mut node = 0;

    for (i, &byte) in text.iter().enumerate() {
      let b = byte as usize;

      // Follow failure links until we find a valid transition or reach the root
      while node != 0 && self.nodes[node].transition[b].is_none() {
        node = self.nodes[node].failure;
      }

      // Transition to the next node (or root if no transition exists)
      node = self.nodes[node].transition[b].unwrap_or(ROOT);

      // Record matches for all patterns that end at this node
      for &pattern_idx in &self.nodes[node].output {
        let start = i + 1 - self.pattern_lengths[pattern_idx];
        matches[pattern_idx].push(start);
      }
    }

    matches
  }
}
