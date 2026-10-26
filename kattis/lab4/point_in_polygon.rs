use std::fmt::Write;

use popup::geometry::{Containment, Point, Polygon};
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

    let m = gm.next();
    for _ in 0..m {
      let x: i32 = gm.next();
      let y: i32 = gm.next();
      let p = Point::new(x, y);

      let result = match poly.contains(&p) {
        Containment::Inside => "in",
        Containment::Boundary => "on",
        Containment::Outside => "out",
      };

      writeln!(out, "{}", result).unwrap();
    }
  }

  print!("{}", out);
}
