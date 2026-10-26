pub struct Interval<T> {
  pub start: T,
  pub end: T,
}

impl<T: PartialOrd> Interval<T> {
  /// Create a new interval with the given start and end points, ensuring that
  /// the start is less than or equal to the end.
  pub fn new(start: T, end: T) -> Self {
    assert!(
      start <= end,
      "Interval start must be less than or equal to end"
    );
    Interval { start, end }
  }
}

/// Given a target interval and a list of intervals, find the smallest subset of
/// intervals that covers the target interval.
///
/// The interval cover is returned as a vector of indices corresponding to the
/// input intervals. If no cover exists, return `None`.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
///
/// where *n* is the number of intervals.
pub fn cover<T>(target: Interval<T>, intervals: &[Interval<T>]) -> Option<Vec<usize>>
where
  T: PartialOrd + Copy,
{
  // Handle degenerate case (target is a point)
  if target.start == target.end {
    for (i, interval) in intervals.iter().enumerate() {
      if interval.start <= target.start && interval.end >= target.end {
        return Some(vec![i]);
      }
    }
    return None; // No interval covers the point
  }

  // Save the original indices of the intervals for output
  let mut indexed: Vec<_> = intervals.iter().enumerate().collect();

  // T must in essence implement Ord. But floats don't, hence the hacky approach
  indexed.sort_by(|&(_, a), &(_, b)| {
    a.start
      .partial_cmp(&b.start)
      .expect("Interval endpoints must be comparable")
  });

  let mut covered_until = target.start;
  let mut result = Vec::new();
  let mut i = 0;

  while covered_until < target.end {
    let mut best_end = covered_until;
    let mut best_idx = None;

    // Find the interval that extends coverage as far as possible
    while i < indexed.len() && indexed[i].1.start <= covered_until {
      if indexed[i].1.end > best_end {
        best_end = indexed[i].1.end;
        best_idx = Some(indexed[i].0);
      }
      i += 1;
    }

    result.push(best_idx?); // If no interval can extend the coverage, return None
    covered_until = best_end;
  }

  Some(result)
}
