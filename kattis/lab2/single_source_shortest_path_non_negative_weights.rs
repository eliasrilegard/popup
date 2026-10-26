use std::fmt::Write;

use popup::graph::{Distance, Graph, dijkstra};
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
    let s = gm.next();

    let mut graph = Graph::new(n);
    for _ in 0..m {
      let u = gm.next();
      let v = gm.next();
      let w: u32 = gm.next();
      graph.add_edge(u, v, w);
    }

    let result = dijkstra(&graph, s);

    for _ in 0..q {
      let t: usize = gm.next();
      match result.distance[t] {
        Distance::Finite(d) => writeln!(out, "{}", d).unwrap(),
        Distance::Unreachable => out.push_str("Impossible\n"),
        _ => unreachable!(),
      }
    }
  }

  print!("{}", out);
}
