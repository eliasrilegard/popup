use std::fmt::Write;

use popup::graph::{Distance, Graph, floyd_warshall};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  while let Some(n) = gm.next_opt() {
    if n == 0 {
      break;
    }

    let m = gm.next();
    let q = gm.next();

    let mut graph = Graph::new(n);
    for _ in 0..m {
      let u = gm.next();
      let v = gm.next();
      let w: i32 = gm.next();
      graph.add_edge(u, v, w);
    }

    let distances = floyd_warshall(&graph);

    for _ in 0..q {
      let s: usize = gm.next();
      let t: usize = gm.next();
      match distances[(s, t)] {
        Distance::Finite(d) => writeln!(out, "{}", d).unwrap(),
        Distance::Unreachable => out.push_str("Impossible\n"),
        Distance::NegativeCycle => out.push_str("-Infinity\n"),
      }
    }
    out.push('\n');
  }

  println!("{}", out.trim_end());
}
