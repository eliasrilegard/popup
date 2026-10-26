/// Construct the suffix array of `s`.
///
/// The suffix array contains the starting byte offset of each suffix of `s` in
/// lexicographical order.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
///
/// where *n* is the length of `s`.
pub fn suffix_array(s: impl AsRef<[u8]>) -> Vec<usize> {
  let s = s.as_ref();
  let n = s.len();

  if n == 0 {
    return Vec::new();
  }

  let mut sa: Vec<usize> = (0..n).collect();
  let mut rank: Vec<usize> = s.iter().map(|&c| c as usize).collect();
  let mut new_rank = vec![0; n];

  let mut k = 1;
  while k < n {
    // Radix sort suffixes based on the first 2k characters.
    //
    // rank[i] identifies the equivalence class of the first k characters of
    // s[i..]. The next k characters of s[i..] are the first k characters of
    // s[i+k..], identified by rank[i+k]. Thus the pair (rank[i], rank[i+k])
    // identifies the first 2k characters of s[i..].
    counting_sort(&mut sa, &rank, k);
    counting_sort(&mut sa, &rank, 0);

    // Reassign new ranks
    new_rank[sa[0]] = 0;
    for i in 1..n {
      let prev = sa[i - 1];
      let curr = sa[i];

      let prev_rank = (rank[prev], rank.get(prev + k));
      let curr_rank = (rank[curr], rank.get(curr + k));

      // If the first 2k characters of s[prev..] and s[curr..] are the same,
      // then they get the same new rank. Otherwise, the new rank of s[curr..]
      // is one more than the new rank of s[prev..].
      new_rank[curr] = new_rank[prev] + if prev_rank == curr_rank { 0 } else { 1 };
    }

    std::mem::swap(&mut rank, &mut new_rank);

    if rank[sa[n - 1]] == n - 1 {
      break; // All suffixes are uniquely ranked
    }

    k *= 2;
  }

  sa
}

/// Sorts the suffix array `sa` based on the ranks of the suffixes starting at
/// `offset` characters ahead.
fn counting_sort(sa: &mut [usize], rank: &[usize], offset: usize) {
  let n = sa.len();

  // Bucket 0 is reserved for the missing second half, and real ranks then
  // get shifted by 1
  let bucket_count = rank.iter().max().map_or(0, |&r| r + 2);
  let mut count = vec![0; bucket_count];

  // Count the occurrences of each rank (with offset) in the suffix array
  for &i in sa.iter() {
    let key = rank.get(i + offset).map_or(0, |&r| r + 1);
    count[key] += 1;
  }

  // Compute the starting index for each rank in the sorted order
  for i in 1..bucket_count {
    count[i] += count[i - 1];
  }

  // Build the sorted suffix array based on the rank (with offset)
  let mut output = vec![0; n];
  for &i in sa.iter().rev() {
    let key = rank.get(i + offset).map_or(0, |&r| r + 1);
    count[key] -= 1;
    output[count[key]] = i;
  }

  sa.copy_from_slice(&output);
}
