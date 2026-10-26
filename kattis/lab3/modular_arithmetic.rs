use std::fmt::Write;

use popup::io::Gimme;
use popup::number_theory::{mod_div, mod_mul};

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    if n == 0 {
      break;
    }

    let t = gm.next();
    for _ in 0..t {
      let x: i64 = gm.next();
      let op = gm.next_byte();
      let y: i64 = gm.next();

      let result = match op {
        b'+' => (x + y).rem_euclid(n),
        b'-' => (x - y).rem_euclid(n),
        b'*' => mod_mul(x, y, n),
        b'/' => mod_div(x, y, n).unwrap_or(-1),
        _ => unreachable!(),
      };

      writeln!(out, "{}", result).unwrap();
    }
  }

  print!("{}", out);
}
