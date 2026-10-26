use popup::graph::{Graph, min_cost_max_flow};
use popup::io::Gimme;

fn main() {
  let mut gm = Gimme::new();

  let n = gm.next();
  let m = gm.next();
  let s = gm.next();
  let t = gm.next();

  let mut network = Graph::new(n);
  for _ in 0..m {
    let u = gm.next();
    let v = gm.next();
    let c = gm.next();
    let w = gm.next();

    network.add_flow_edge_with_cost(u, v, c, w);
  }

  let mcmf = min_cost_max_flow(&network, s, t);

  println!("{} {}", mcmf.total_flow, mcmf.total_cost);
}
