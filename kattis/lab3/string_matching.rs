use std::fmt::Write;

use popup::io::Gimme;
use popup::string::kmp;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while gm.has_more() {
    let patten = gm.next_line().to_owned();
    let haystack = gm.next_line();

    let matches = kmp(&patten, haystack);

    let strs: Vec<_> = matches.iter().map(|&i| i.to_string()).collect();
    writeln!(out, "{}", strs.join(" ")).unwrap();
  }

  print!("{}", out);
}
