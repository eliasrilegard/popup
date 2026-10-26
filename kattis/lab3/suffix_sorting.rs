use std::fmt::Write;

use popup::io::Gimme;
use popup::string::suffix_array;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while gm.has_more() {
    let s = gm.next_line().to_owned();
    let n = gm.next();
    let queries: Vec<usize> = (0..n).map(|_| gm.next()).collect();

    let sa = suffix_array(&s);

    let strs: Vec<_> = queries.iter().map(|&q| sa[q].to_string()).collect();
    writeln!(out, "{}", strs.join(" ")).unwrap();
  }

  print!("{}", out);
}
