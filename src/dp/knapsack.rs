pub struct Item {
  pub value: usize,
  pub weight: usize,
}

/// Solves the 0/1 Knapsack problem using dynamic programming.
///
/// # Complexity
/// * Time complexity: *O(n * c)*.
/// * Auxiliary space: *O(n * c)*.
///
/// where `n` is the number of items and `c` is the maximum weight capacity of
/// the knapsack.
pub fn knapsack(capacity: usize, items: &[Item]) -> Vec<usize> {
  let n = items.len();
  let mut dp = vec![vec![0; capacity + 1]; n + 1];

  for i in 1..=n {
    for w in 0..=capacity {
      let weight = items[i - 1].weight;
      let value = items[i - 1].value;
      if weight > w {
        dp[i][w] = dp[i - 1][w];
      } else {
        dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - weight] + value);
      }
    }
  }

  // Backtrack to find the items included in the optimal solution
  let mut w = capacity;
  let mut selected = Vec::new();
  for i in (1..=n).rev() {
    if dp[i][w] != dp[i - 1][w] {
      selected.push(i - 1);
      w -= items[i - 1].weight;
    }
  }

  selected.reverse(); // Maintain original order
  selected
}
