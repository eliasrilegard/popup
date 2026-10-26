//! Linear algebra algorithms and data types.

mod gaussian;
mod matrix;
mod solver;

use gaussian::reduced_row_echelon;
pub use matrix::Matrix;
pub use solver::{LinearSolution, solve_system};
