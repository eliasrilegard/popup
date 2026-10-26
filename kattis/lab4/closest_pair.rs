use std::fmt::Write;

use popup::geometry::{Point, closest_pair};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    if n == 0 {
      break;
    }

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
      let x: f64 = gm.next();
      let y = gm.next();
      points.push(Point::new(x, y));
    }

    let pair = closest_pair(&points).unwrap();
    let p = points[pair.0];
    let q = points[pair.1];

    writeln!(out, "{:.2} {:.2} {:.2} {:.2}", p.x, p.y, q.x, q.y).unwrap();
  }

  print!("{}", out);
}
