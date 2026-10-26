use std::iter::once;

use crate::linear_algebra::Matrix;

/// Represent an affine recurrence relation of the form
/// `x[t] = a_0 + \sum_{i=1}^N a_i x[t-i]`
pub struct AffineRecurrence {
  /// The constant term a_0 in the recurrence.
  constant: i64,
  /// The coefficients a_1, ..., a_N in the recurrence.
  coeffs: Vec<i64>,
  /// The initial values x[0], ..., x[N-1] of the recurrence.
  initial: Vec<i64>,
}

impl AffineRecurrence {
  /// Create an affine recurrence relation with the given constant term,
  /// coefficients, and initial values.
  ///
  /// The lengths of `coeffs` and `initial` must match; both determine the
  /// degree of the recurrence.
  pub fn new(constant: i64, coeffs: Vec<i64>, initial: Vec<i64>) -> Self {
    assert_eq!(coeffs.len(), initial.len());
    Self {
      constant,
      coeffs,
      initial,
    }
  }

  /// Create a linear recurrence relation with the given coefficients and
  /// initial values.
  ///
  /// This is equivalent to an affine recurrence with a constant term of zero.
  pub fn new_linear(coeffs: Vec<i64>, initial: Vec<i64>) -> Self {
    Self::new(0, coeffs, initial)
  }

  /// Determine the `n`-th term of the recurrence modulo `modulus`.
  ///
  /// # Complexity
  /// * Time complexity: *O(k^3 log n)*.
  /// * Auxiliary space: *O(k^2)*.
  ///
  /// where *k* is the degree of the recurrence.
  pub fn nth(&self, n: u64, modulus: i64) -> i64 {
    let k = self.coeffs.len();

    if n < k as u64 {
      return self.initial[n as usize].rem_euclid(modulus);
    }

    let matrix = self.transition_matrix(modulus);

    let power = n - (k as u64 - 1);
    let matrix_exp = pow_mod(&matrix, power, modulus);

    let state = self.initial_vector(modulus);

    let result = mul_mod(&matrix_exp, &state, modulus);
    result[(0, 0)]
  }

  /// Return the transition matrix of the affine recurrence modulo `modulus`.
  fn transition_matrix(&self, modulus: i64) -> Matrix<i64> {
    let k = self.coeffs.len();
    let mut a = Matrix::zeros(k + 1, k + 1);

    // Top row: a1, ..., ak, a0
    for i in 0..k {
      a[(0, i)] = self.coeffs[i].rem_euclid(modulus);
    }
    a[(0, k)] = self.constant.rem_euclid(modulus);

    // Subdiagonal
    for i in 1..k {
      a[(i, i - 1)] = 1;
    }

    // Last row keeps constant 1 (affine)
    a[(k, k)] = 1;

    a
  }

  /// Return the initial state vector of the affine recurrence modulo `modulus`.
  ///
  /// The initial state vector has the form
  /// `[x[k-1], x[k-2], ..., x[0], 1]^T`, where k is the degree of the
  /// recurrence.
  fn initial_vector(&self, modulus: i64) -> Matrix<i64> {
    let k = self.coeffs.len();
    let data = self
      .initial
      .iter()
      .rev()
      .map(|&x| x.rem_euclid(modulus))
      .chain(once(1))
      .collect();
    Matrix::new(k + 1, 1, data)
  }
}

/// Return `matrix` to the power of `exp`, modulo `modulus`.
///
/// Time complexity: *O(n^3 log exp)*, where *n* is the dimension of the square
/// matrix.
fn pow_mod(matrix: &Matrix<i64>, mut exp: u64, modulus: i64) -> Matrix<i64> {
  let n = matrix.rows();
  assert_eq!(n, matrix.cols(), "Matrix must be square for exponentiation");

  let mut result = Matrix::zeros(n, n);
  for i in 0..n {
    result[(i, i)] = 1;
  }

  let mut base = matrix.clone();
  while exp > 0 {
    if exp & 1 == 1 {
      result = mul_mod(&result, &base, modulus);
    }
    base = mul_mod(&base, &base, modulus);
    exp >>= 1;
  }

  result
}

/// Multiply matrices `a` and `b` modulo `modulus`.
///
/// Time complexity: *O(nmp)*, where *n* is the number of rows in `a`, *m* is
/// the number of columns in `a` (and rows in `b`), and *p* is the number of
/// columns in `b`.
fn mul_mod(a: &Matrix<i64>, b: &Matrix<i64>, modulus: i64) -> Matrix<i64> {
  let n = a.rows();
  let m = a.cols();
  let p = b.cols();
  assert_eq!(
    m,
    b.rows(),
    "Matrix dimensions must be compatible for multiplication"
  );

  let mut result = Matrix::zeros(n, p);

  for i in 0..n {
    for j in 0..p {
      let mut sum = 0;
      for k in 0..m {
        sum = (sum + a[(i, k)] * b[(k, j)]).rem_euclid(modulus);
      }
      result[(i, j)] = sum;
    }
  }

  result
}
