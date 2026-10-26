use crate::number_theory::egcd;

/// Represents a congruence of the form `x ≡ remainder (mod modulus)`.
pub struct Congruence {
  pub remainder: i64,
  pub modulus: i64,
}

impl Congruence {
  /// Create a new congruence with the given remainder and modulus.
  pub fn new(remainder: i64, modulus: i64) -> Self {
    Self {
      remainder: remainder.rem_euclid(modulus),
      modulus,
    }
  }
}

impl From<(i64, i64)> for Congruence {
  fn from((remainder, modulus): (i64, i64)) -> Self {
    Self::new(remainder, modulus)
  }
}

/// Solve a system of congruences using the generalized Chinese Remainder
/// Theorem.
///
/// Return a single [`Congruence`] equivalent to all input congruences, or
/// `None` if the system has no solution. The resulting remainder is normalized
/// to `0..modulus`.
///
/// The moduli need not be pairwise coprime. Two congruences can be merged
/// precisely when their remainders agree modulo the greatest common divisor
/// of their moduli.
///
/// Time complexity: *O(n log M)*, where *n* is the number of congruences and
/// *M* is their maximum modulus.
pub fn solve_crt<I, C>(congruences: I) -> Option<Congruence>
where
  I: IntoIterator<Item = C>,
  C: Into<Congruence>,
{
  let mut iter = congruences.into_iter().map(Into::into);
  let first = iter.next()?;
  iter.try_fold(first, merge_crt)
}

/// Merge two congruences into an equivalent single congruence using the
/// generalized Chinese Remainder Theorem.
///
/// Return `None` if the two congruences are incompatible.
///
/// Time complexity: *O(log max(m1, m2))* where *m1* and *m2* are the moduli of
/// the two congruences.
///
/// # Explanation
/// ```text
/// z ≡ r1 (mod m1) yields z = r1 + m1 * t for some t.
/// z ≡ r2 (mod m2) yields m1 * t ≡ r2 - r1 = d (mod m2).
///
/// The second congruence has solution iff gcd(m1, m2) divides d.
///
/// The Extended Euclidean algorithm gives
///   m1 * x + m2 * y = g,
/// i.e.,
///   m1 / g * x ≡ 1 (mod m2/g).
///
/// From m1 * t ≡ d (mod m2), dividing through by g gives
///   (m1 / g) * t ≡ d / g (mod m2/g).
///
/// Since m1 / g is coprime to m2 / g and has inverse x, we can multiply both
/// sides by x to get
///   t ≡ (d / g) * x (mod m2/g).
///
/// Then we can substitute back to get
///   z ≡ a1 + m1 * t (mod lcm(m1, m2)).
/// ```
fn merge_crt(a: Congruence, b: Congruence) -> Option<Congruence> {
  let (gcd, x, _) = egcd(a.modulus, b.modulus);

  // System is solvable iff a1 ≡ a2 (mod g)
  if (b.remainder - a.remainder) % gcd != 0 {
    return None; // No solution exists
  }

  // Merge into single congruence x ≡ r_merged (mod lcm(m1, m2))
  let lcm = a.modulus / gcd * b.modulus;
  let t = ((b.remainder - a.remainder) / gcd * x).rem_euclid(b.modulus / gcd);
  let r_merged = (a.remainder + a.modulus * t).rem_euclid(lcm);

  Some(Congruence {
    remainder: r_merged,
    modulus: lcm,
  })
}
