use std::fmt::Write;

use popup::graph::{Graph, hierholzer};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    if n == 0 {
      break;
    }

    let m = gm.next();

    let mut graph = Graph::new(n);
    for _ in 0..m {
      let u = gm.next();
      let v = gm.next();

      graph.add_edge(u, v, ());
    }

    match hierholzer(&graph) {
      Some(path) => {
        let strs: Vec<_> = path.iter().map(|u| u.to_string()).collect();
        writeln!(out, "{}", strs.join(" ")).unwrap();
      }
      None => out.push_str("Impossible\n"),
    }
  }

  print!("{}", out);
}
