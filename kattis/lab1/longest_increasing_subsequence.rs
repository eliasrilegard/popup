use std::fmt::Write;

use popup::dp::lis;
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    let values: Vec<usize> = (0..n).map(|_| gm.next()).collect();

    let indices = lis(&values);

    writeln!(out, "{}", indices.len()).unwrap();
    let strs: Vec<_> = indices.iter().map(|&i| i.to_string()).collect();
    writeln!(out, "{}", strs.join(" ")).unwrap();
  }

  print!("{}", out);
}
