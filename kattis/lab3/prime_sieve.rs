use std::fmt::Write;

use popup::io::Gimme;
use popup::number_theory::Eratosthenes;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  let q = gm.next();

  let sieve = Eratosthenes::new(n);

  writeln!(out, "{}", sieve.count()).unwrap();
  for _ in 0..q {
    let x = gm.next();
    writeln!(out, "{}", if sieve.is_prime(x) { 1 } else { 0 }).unwrap();
  }

  print!("{}", out);
}
