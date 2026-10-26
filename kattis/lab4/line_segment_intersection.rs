use std::fmt::Write;

use popup::geometry::{Intersection, Point, Segment};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  for _ in 0..n {
    let x1: i32 = gm.next();
    let y1 = gm.next();
    let x2 = gm.next();
    let y2 = gm.next();
    let x3 = gm.next();
    let y3 = gm.next();
    let x4 = gm.next();
    let y4 = gm.next();

    let p1 = Point::new(x1, y1);
    let p2 = Point::new(x2, y2);
    let p3 = Point::new(x3, y3);
    let p4 = Point::new(x4, y4);

    let s1 = Segment::new(p1, p2);
    let s2 = Segment::new(p3, p4);

    match s1.intersects(&s2) {
      Intersection::None => out.push_str("none\n"),
      Intersection::Point(p) => writeln!(out, "{:.2} {:.2}", p.x, p.y).unwrap(),
      Intersection::Segment(s) => {
        let mut points = [s.start, s.end];
        points.sort_by(|a, b| {
          a.x
            .partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
        });
        writeln!(
          out,
          "{:.2} {:.2} {:.2} {:.2}",
          points[0].x, points[0].y, points[1].x, points[1].y
        )
        .unwrap();
      }
    }
  }

  print!("{}", out);
}
