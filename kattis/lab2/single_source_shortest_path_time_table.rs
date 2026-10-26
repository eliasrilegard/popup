use std::fmt::Write;

use popup::graph::{Distance, Graph, dijkstra_with};
use popup::io::Gimme;

struct Timetable {
  first_departure: u32,
  period: u32,
  duration: u32,
}

impl Timetable {
  fn wait_time(&self, arrival: u32) -> Option<u32> {
    if arrival <= self.first_departure {
      Some(self.first_departure - arrival)
    } else if self.period == 0 {
      None
    } else {
      let elapsed = arrival - self.first_departure;
      let wait = (self.period - elapsed % self.period) % self.period;
      Some(wait)
    }
  }
}

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

      let t0 = gm.next();
      let p = gm.next();
      let d = gm.next();

      let tt = Timetable {
        first_departure: t0,
        period: p,
        duration: d,
      };
      graph.add_edge(u, v, tt);
    }

    let result = dijkstra_with(&graph, s, |time, edge| {
      let wait = edge.data.wait_time(time)?;
      Some(wait + edge.data.duration)
    });

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
