use std::fmt::Write;

use popup::ds::UnionFind;
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  let q = gm.next();

  let mut uf = UnionFind::new(n);

  for _ in 0..q {
    let op = gm.next_byte();
    let a = gm.next();
    let b = gm.next();

    match op {
      b'=' => {
        uf.union(a, b);
      }
      b'?' => {
        let res = if uf.connected(a, b) { "yes" } else { "no" };
        writeln!(out, "{}", res).unwrap();
      }
      _ => unreachable!(),
    }
  }

  print!("{}", out);
}
