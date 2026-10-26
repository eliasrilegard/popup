use std::fmt::Write;

use popup::graph::{Graph, prim};
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
      let w: i32 = gm.next();

      graph.add_undirected_edge(u, v, w);
    }

    if let Some(mut mst) = prim(&graph) {
      // Canonicalize edges so that u < v for each edge (u, v, w)
      for (u, v, _) in &mut mst.edges {
        if u > v {
          std::mem::swap(u, v);
        }
      }

      // Sort edges lexicographically
      mst.edges.sort_unstable_by_key(|&(u, v, _)| (u, v));

      writeln!(out, "{}", mst.total_weight).unwrap();
      for (u, v, _) in mst.edges {
        writeln!(out, "{} {}", u, v).unwrap();
      }
    } else {
      out.push_str("Impossible\n");
    }
  }

  print!("{}", out);
}
