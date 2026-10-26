use std::fmt::Write;

use popup::greedy::{Interval, cover};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(a) = gm.next_opt() {
    let b: f32 = gm.next();
    let target = Interval::new(a, b);

    let n = gm.next();
    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
      let a = gm.next();
      let b = gm.next();
      intervals.push(Interval::new(a, b));
    }

    match cover(target, &intervals) {
      Some(indices) => {
        writeln!(out, "{}", indices.len()).unwrap();
        let strs: Vec<_> = indices.iter().map(|&i| i.to_string()).collect();
        writeln!(out, "{}", strs.join(" ")).unwrap();
      }
      None => {
        writeln!(out, "impossible").unwrap();
      }
    }
  }

  print!("{}", out);
}
