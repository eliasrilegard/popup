use std::ops::{Index, IndexMut};

use crate::algebra::{One, Zero};

#[derive(Clone)]
pub struct Matrix<T> {
  rows: usize,
  cols: usize,
  data: Vec<T>,
}

#[cfg(debug_assertions)]
impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for r in 0..self.rows {
      for c in 0..self.cols {
        write!(f, "{:?} ", self[(r, c)])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
    &self.data[r * self.cols + c]
  }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
    &mut self.data[r * self.cols + c]
  }
}

impl<T> Matrix<T> {
  pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
    assert_eq!(
      rows * cols,
      data.len(),
      "Provided data does not match the specified dimensions"
    );
    Matrix { rows, cols, data }
  }

  pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
    let rows_count = rows.len();
    let cols_count = if rows_count > 0 { rows[0].len() } else { 0 };
    let mut data = Vec::with_capacity(rows_count * cols_count);

    for row in rows {
      assert_eq!(
        row.len(),
        cols_count,
        "All rows must have the same number of columns"
      );
      data.extend(row);
    }

    Matrix {
      rows: rows_count,
      cols: cols_count,
      data,
    }
  }

  pub fn rows(&self) -> usize {
    self.rows
  }

  pub fn cols(&self) -> usize {
    self.cols
  }

  pub fn row(&self, row: usize) -> &[T] {
    let start = row * self.cols;
    let end = start + self.cols;
    &self.data[start..end]
  }

  pub fn row_mut(&mut self, row: usize) -> &mut [T] {
    let start = row * self.cols;
    let end = start + self.cols;
    &mut self.data[start..end]
  }

  pub fn swap_rows(&mut self, row1: usize, row2: usize) {
    if row1 == row2 {
      return;
    }
    for col in 0..self.cols {
      self
        .data
        .swap(row1 * self.cols + col, row2 * self.cols + col);
    }
  }

  pub fn augment(&self, other: &Matrix<T>) -> Self
  where
    T: Clone,
  {
    assert_eq!(
      self.rows, other.rows,
      "Matrices must have the same number of rows to augment"
    );

    let new_cols = self.cols + other.cols;
    let mut new_data = Vec::with_capacity(self.rows * new_cols);

    for r in 0..self.rows {
      new_data.extend_from_slice(self.row(r));
      new_data.extend_from_slice(other.row(r));
    }

    Self {
      rows: self.rows,
      cols: new_cols,
      data: new_data,
    }
  }

  pub fn zeros(rows: usize, cols: usize) -> Self
  where
    T: Zero + Clone,
  {
    Self {
      rows,
      cols,
      data: vec![T::ZERO; rows * cols],
    }
  }

  pub fn identity(size: usize) -> Self
  where
    T: Zero + One + Clone,
  {
    let mut data = vec![T::ZERO; size * size];
    for i in 0..size {
      data[i * size + i] = T::ONE;
    }

    Self {
      rows: size,
      cols: size,
      data,
    }
  }
}
