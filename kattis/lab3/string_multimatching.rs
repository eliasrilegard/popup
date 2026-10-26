use std::fmt::Write;

use popup::io::Gimme;
use popup::string::AhoCorasick;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    let patterns: Vec<_> = (0..n).map(|_| gm.next_line().to_owned()).collect();
    let haystack = gm.next_line();

    let ac = AhoCorasick::new(&patterns);
    let matches = ac.find_all(haystack);

    for positions in matches {
      let strs: Vec<_> = positions.iter().map(|&p| p.to_string()).collect();
      writeln!(out, "{}", strs.join(" ")).unwrap();
    }
  }

  print!("{}", out);
}
