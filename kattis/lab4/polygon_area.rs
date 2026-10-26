use std::fmt::Write;

use popup::geometry::{Point, Polygon};
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
      let x: i32 = gm.next();
      let y: i32 = gm.next();
      points.push(Point::new(x, y));
    }

    let poly = Polygon::new(points);
    let area = poly.signed_area();

    match area {
      a if a > 0.0 => writeln!(out, "CCW {:.1}", a).unwrap(),
      a if a < 0.0 => writeln!(out, "CW {:.1}", -a).unwrap(),
      _ => writeln!(out, "N/A 0.0").unwrap(),
    }
  }

  print!("{}", out);
}
