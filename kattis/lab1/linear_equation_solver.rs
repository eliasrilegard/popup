use std::fmt::Write;

use popup::io::Gimme;
use popup::linear_algebra::{LinearSolution, Matrix, solve_system};

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    if n == 0 {
      break;
    }

    let mut a = Matrix::zeros(n, n);
    let mut b = vec![0.0; n];

    for i in 0..n {
      for j in 0..n {
        a[(i, j)] = gm.next();
      }
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
      b[i] = gm.next();
    }

    match solve_system(&a, &b) {
      LinearSolution::Unique(x) => {
        let strs: Vec<_> = x.iter().map(|&v| format!("{:.2}", v)).collect();
        writeln!(out, "{}", strs.join(" ")).unwrap();
      }
      LinearSolution::Infinite(space) => {
        let strs: Vec<_> = space
          .particular
          .iter()
          .enumerate()
          .map(|(i, &v)| {
            if space.is_free(i) {
              "?".to_string()
            } else {
              format!("{:.2}", v)
            }
          })
          .collect();
        writeln!(out, "{}", strs.join(" ")).unwrap();
      }
      LinearSolution::Inconsistent => {
        writeln!(out, "inconsistent").unwrap();
      }
    }
  }

  print!("{}", out);
}
