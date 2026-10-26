use popup::graph::{Graph, min_cut};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();

  let n = gm.next();
  let m = gm.next();
  let s = gm.next();
  let t = gm.next();

  let mut graph = Graph::new(n);
  for _ in 0..m {
    let u = gm.next();
    let v = gm.next();
    let w = gm.next();

    graph.add_flow_edge(u, v, w);
  }

  let mc = min_cut(&graph, s, t);

  let strs: Vec<_> = mc.iter().map(|u| u.to_string()).collect();
  println!("{}\n{}", mc.len(), strs.join("\n"));
}
