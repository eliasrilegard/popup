use std::fmt::Write;

use popup::io::Gimme;
use popup::number_theory::solve_crt;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let t = gm.next();
  for _ in 0..t {
    let a = gm.next();
    let n = gm.next();
    let b = gm.next();
    let m = gm.next();

    if let Some(merged) = solve_crt([(a, n), (b, m)]) {
      writeln!(out, "{} {}", merged.remainder, merged.modulus).unwrap();
    } else {
      out.push_str("no solution\n");
    }
  }

  print!("{}", out);
}
