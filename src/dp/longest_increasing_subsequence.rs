/// Compute the indices of the longest increasing subsequence (LIS) of a slice
/// of values.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
pub fn lis<T: Ord>(values: &[T]) -> Vec<usize> {
  let n = values.len();

  // tails[k] is the index of the smallest possible tail value of any increasing
  // subsequence of length k+1 seen so far
  let mut tails = Vec::with_capacity(n);
  let mut prev = vec![None; n];

  for i in 0..n {
    // Find the smallest ℓ such that the tail of an increasing subsequence of
    // length ℓ+1 is >= values[i]
    let pos = tails.partition_point(|&j| values[j] < values[i]);

    // Record predecessor before updating tails
    if pos > 0 {
      prev[i] = Some(tails[pos - 1]);
    }

    match tails.get_mut(pos) {
      Some(slot) => *slot = i,
      None => tails.push(i),
    }
  }

  // (Re)construct the LIS
  let mut result = Vec::with_capacity(tails.len());
  if let Some(mut k) = tails.last().copied() {
    result.push(k);

    // Follow predecessor chain
    while let Some(p) = prev[k] {
      k = p;
      result.push(p);
    }

    result.reverse(); // Maintain original order
  }

  result
}
