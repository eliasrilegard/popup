use std::fmt::Write;

use popup::algebra::convolve;
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let t = gm.next(); // Always 1
  for _ in 0..t {
    let n = gm.next();
    let a: Vec<i64> = (0..=n).map(|_| gm.next()).collect();

    let m = gm.next();
    let b: Vec<i64> = (0..=m).map(|_| gm.next()).collect();

    let c = convolve(&a, &b);

    writeln!(out, "{}", c.len() - 1).unwrap();
    let fmt: Vec<_> = c.iter().map(|x| x.to_string()).collect();
    writeln!(out, "{}", fmt.join(" ")).unwrap();
  }

  print!("{}", out);
}
