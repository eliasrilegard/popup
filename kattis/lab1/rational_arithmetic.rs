use std::fmt::Write;

use popup::algebra::Rational;
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  for _ in 0..n {
    let x1 = gm.next();
    let y1 = gm.next();
    let op = gm.next_byte();
    let x2 = gm.next();
    let y2 = gm.next();

    let r1 = Rational::new(x1, y1);
    let r2 = Rational::new(x2, y2);

    let result = match op {
      b'+' => r1 + r2,
      b'-' => r1 - r2,
      b'*' => r1 * r2,
      b'/' => r1 / r2,
      _ => unreachable!(),
    };

    writeln!(out, "{} / {}", result.num(), result.den()).unwrap();
  }

  print!("{}", out);
}
