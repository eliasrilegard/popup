use std::collections::HashMap;
use std::fmt::Write;

use popup::graph::{Graph, edmonds_karp};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();
  let mut out = String::new();

  let n = gm.next();
  let m = gm.next();
  let s = gm.next();
  let t = gm.next();

  let mut network = Graph::new(n);
  for _ in 0..m {
    let u = gm.next();
    let v = gm.next();
    let c = gm.next();

    network.add_flow_edge(u, v, c);
  }

  let mf = edmonds_karp(&network, s, t);

  // Conform to Kattis
  let mut aggregated = HashMap::new();
  for flow in mf.flows {
    *aggregated.entry((flow.from, flow.to)).or_insert(0) += flow.amount;
  }

  writeln!(out, "{} {} {}", n, mf.total_flow, aggregated.len()).unwrap();
  for ((from, to), amount) in aggregated {
    writeln!(out, "{} {} {}", from, to, amount).unwrap();
  }

  print!("{}", out);
}
