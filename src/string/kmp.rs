/// Finds all occurrences of pattern in text using the Knuth–Morris–Pratt
/// (KMP) string-matching algorithm.
///
/// Returns the starting byte offset of each occurrence, including overlapping
/// occurrences. An empty pattern is considered to match at every byte
/// boundary in text.
///
/// # Complexity
/// * Time complexity: *O(n + m)*.
/// * Auxiliary space: *O(m)* for the LPS array.
///
/// where *n* is the text length and *m* is the pattern length.
pub fn kmp(pattern: impl AsRef<[u8]>, text: impl AsRef<[u8]>) -> Vec<usize> {
  let pattern = pattern.as_ref();
  let text = text.as_ref();

  // Edge case: empty pattern matches at every position in the text
  if pattern.is_empty() {
    return (0..=text.len()).collect();
  }

  let m = pattern.len();
  let lps = build_lps(pattern);

  let mut matches = Vec::new();
  let mut j = 0; // Index in pattern

  for (i, &byte) in text.iter().enumerate() {
    // If there's a mismatch, fall back in the pattern using the LPS array
    while j > 0 && byte != pattern[j] {
      j = lps[j - 1];
    }
    if byte == pattern[j] {
      j += 1;
    }
    if j == m {
      matches.push(i + 1 - m);
      j = lps[j - 1]; // Allow for overlapping matches
    }
  }

  matches
}

/// Computes the longest prefix-suffix (LPS) array for the given byte slice.
///
/// `lps[i]` is the length of the longest proper prefix of `bytes[..=i]` which
/// is also a suffix of `bytes[..=i]`.
///
/// Time complexity: *O(n)*, where *n* is the slice length.
fn build_lps(bytes: &[u8]) -> Vec<usize> {
  let n = bytes.len();
  let mut lps = vec![0; n];

  for i in 1..n {
    let mut j = lps[i - 1];
    while j > 0 && bytes[i] != bytes[j] {
      j = lps[j - 1];
    }
    if bytes[i] == bytes[j] {
      j += 1;
    }
    lps[i] = j;
  }

  lps
}
