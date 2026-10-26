use std::fmt::Write;

use popup::io::Gimme;
use popup::number_theory::AffineRecurrence;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();

  let a0: i64 = gm.next();
  let a: Vec<i64> = (0..n).map(|_| gm.next()).collect();
  let x: Vec<i64> = (0..n).map(|_| gm.next()).collect();

  let rec = AffineRecurrence::new(a0, a, x);

  let q = gm.next();
  for _ in 0..q {
    let t = gm.next();
    let m = gm.next();

    let result = rec.nth(t, m);
    writeln!(out, "{}", result).unwrap();
  }

  print!("{}", out);
}
