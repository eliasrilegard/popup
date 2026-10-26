use crate::algebra::Field;
use crate::linear_algebra::Matrix;

/// A struct representing the reduced row echelon form of a matrix, along with
/// the pivot positions.
pub struct ReducedRowEchelon<T> {
  pub matrix: Matrix<T>,
  pub pivot_cols: Vec<usize>, // Row i has pivot in column pivots[i]
}

/// Compute the reduced row echelon form of a matrix using Gauss–Jordan
/// elimination.
///
/// # Complexity
/// * Time complexity: *O(m * n * min(m, n))*.
/// * Auxiliary space: *O(1)* (in-place).
pub fn reduced_row_echelon<T>(mut matrix: Matrix<T>) -> ReducedRowEchelon<T>
where
  T: Field + Clone,
{
  let mut pivots = Vec::new();

  for col in 0..matrix.cols() {
    // Find the pivot row for this column
    let Some(pivot_row) = find_pivot(&matrix, pivots.len(), col) else {
      continue;
    };

    // Swap rows to move the pivot into position
    matrix.swap_rows(pivots.len(), pivot_row);

    // Normalize the pivot row and eliminate the column
    normalize_row(&mut matrix, pivots.len(), col);
    eliminate_column(&mut matrix, pivots.len(), col);

    pivots.push(col);
  }

  ReducedRowEchelon {
    matrix,
    pivot_cols: pivots,
  }
}

// Find the pivot row for a given column, starting from a given row index.
// Return None if no pivot is found.
fn find_pivot<T>(matrix: &Matrix<T>, row_start: usize, col: usize) -> Option<usize>
where
  T: Field,
{
  (row_start..matrix.rows()).find(|&row| !matrix[(row, col)].is_zero())
}

// Normalize the pivot row by dividing each element by the pivot value.
fn normalize_row<T>(matrix: &mut Matrix<T>, row: usize, pivot_col: usize)
where
  T: Field + Clone,
{
  let pivot_value = matrix[(row, pivot_col)].clone();
  for col in pivot_col..matrix.cols() {
    matrix[(row, col)] = matrix[(row, col)].clone() / pivot_value.clone();
  }
}

// Eliminate the pivot column from all other rows by subtracting a multiple of
// the pivot row.
fn eliminate_column<T>(matrix: &mut Matrix<T>, pivot_row: usize, pivot_col: usize)
where
  T: Field + Clone,
{
  for row in 0..matrix.rows() {
    if row == pivot_row {
      continue;
    }

    let factor = matrix[(row, pivot_col)].clone();
    if factor.is_zero() {
      continue;
    }

    for col in 0..matrix.cols() {
      matrix[(row, col)] =
        matrix[(row, col)].clone() - factor.clone() * matrix[(pivot_row, col)].clone();
    }
  }
}
