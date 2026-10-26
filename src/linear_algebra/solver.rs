use crate::algebra::Field;
use crate::linear_algebra::{Matrix, reduced_row_echelon};

/// Represents the solution of a linear system Ax = b
pub enum LinearSolution<T> {
  Unique(Vec<T>),
  Infinite(SolutionSpace<T>),
  Inconsistent,
}

pub struct SolutionSpace<T> {
  pub particular: Vec<T>,
  pub basis: Vec<Vec<T>>,
}

impl<T: Field> SolutionSpace<T> {
  /// Check if a variable is free (not fixed) in the solution space.
  pub fn is_free(&self, variable: usize) -> bool {
    self.basis.iter().any(|vec| !vec[variable].is_zero())
  }
}

/// Solve the linear system Ax = b
///
/// # Complexity
/// * Time complexity: *O(m * n * min(m, n))*.
/// * Auxiliary space: *O(m * n)* for the augmented matrix.
///
/// where *m* is the number of equations and *n* is the number of variables.
pub fn solve_system<T>(a: &Matrix<T>, b: &[T]) -> LinearSolution<T>
where
  T: Field + Clone,
{
  let b_mat = Matrix::from_rows(b.iter().map(|x| vec![x.clone()]).collect());
  let augmented = a.augment(&b_mat);
  let rref = reduced_row_echelon(augmented);

  if is_inconsistent(&rref.matrix) {
    return LinearSolution::Inconsistent;
  }

  let particular = particular_solution(&rref.matrix, &rref.pivot_cols);

  // Check uniqueness
  let free = free_columns(&rref.matrix, &rref.pivot_cols);
  if free.is_empty() {
    return LinearSolution::Unique(particular);
  }

  // Free variables => Infinite solutions
  let basis = basis_vectors(&rref.matrix, &rref.pivot_cols, free);
  LinearSolution::Infinite(SolutionSpace { particular, basis })
}

/// Check if the system is inconsistent by looking for a zero row with a
/// non-zero entry in the last column.
fn is_inconsistent<T>(matrix: &Matrix<T>) -> bool
where
  T: Field + Clone,
{
  (0..matrix.rows()).any(|row| {
    let last_col = matrix.cols() - 1;
    let is_zero_row = (0..last_col).all(|col| matrix[(row, col)].is_zero());
    is_zero_row && !matrix[(row, last_col)].is_zero()
  })
}

/// Return the indices of free columns (non-pivot columns) in the RREF matrix.
fn free_columns<T>(rref: &Matrix<T>, pivots: &[usize]) -> Vec<usize>
where
  T: Field + Clone,
{
  let mut free = Vec::new();

  let mut it = pivots.iter();
  let mut next = it.next();

  for col in 0..rref.cols() - 1 {
    if Some(&col) == next {
      next = it.next();
    } else {
      free.push(col); // Only push non-pivot cols
    }
  }

  free
}

/// Compute a particular solution from the RREF matrix and pivot columns.
fn particular_solution<T>(rref: &Matrix<T>, pivots: &[usize]) -> Vec<T>
where
  T: Field + Clone,
{
  let n = rref.cols() - 1;
  let mut solution = vec![T::ZERO; n];

  for (row, &col) in pivots.iter().enumerate() {
    solution[col] = rref[(row, n)].clone();
  }

  solution
}

/// Compute the basis vectors for the solution space corresponding to free
/// variables.
fn basis_vectors<T>(rref: &Matrix<T>, pivots: &[usize], free: Vec<usize>) -> Vec<Vec<T>>
where
  T: Field + Clone,
{
  let n = rref.cols() - 1;
  let mut basis = Vec::new();

  for &free_col in &free {
    let mut vec = vec![T::ZERO; n];
    vec[free_col] = T::ONE;

    for (row, &pivot_col) in pivots.iter().enumerate() {
      vec[pivot_col] = -rref[(row, free_col)].clone();
    }

    basis.push(vec);
  }

  basis
}
