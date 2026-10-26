use std::fmt::Write;

use popup::dp::{Item, knapsack};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(c) = gm.next_opt() {
    let n = gm.next();

    let mut items = Vec::new();
    for _ in 0..n {
      let value = gm.next();
      let weight = gm.next();
      items.push(Item { value, weight });
    }

    let selected = knapsack(c, &items);

    writeln!(out, "{}", selected.len()).unwrap();
    let strs: Vec<_> = selected.iter().map(|&i| i.to_string()).collect();
    writeln!(out, "{}", strs.join(" ")).unwrap();
  }

  print!("{}", out);
}
