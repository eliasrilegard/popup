use std::fmt::Write;

use popup::io::Gimme;
use popup::string::RollingHash;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let s = gm.next_line().to_owned();

  let rh: RollingHash = RollingHash::new(&s);

  let q = gm.next();
  for _ in 0..q {
    let l = gm.next();
    let r = gm.next();

    let hash = rh.get_hash(l, r);
    writeln!(out, "{}", hash).unwrap();
  }

  print!("{}", out);
}
