use std::fmt::Write;

use popup::ds::FenwickTree;
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  let q = gm.next();

  let mut ft = FenwickTree::new(n, 0);

  for _ in 0..q {
    let op = gm.next_byte();
    match op {
      b'+' => {
        let i = gm.next();
        let d = gm.next();
        ft.add(i, d);
      }
      b'?' => {
        let i = gm.next();
        let sum = ft.prefix_sum(i);
        writeln!(out, "{}", sum).unwrap();
      }
      _ => unreachable!(),
    }
  }

  print!("{}", out);
}
