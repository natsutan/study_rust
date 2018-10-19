extern crate petgraph;

use std::fs;
use std::io::Write;

// https://docs.rs/petgraph/0.4.13/petgraph/
use petgraph::Graph;
use petgraph::algo;
use petgraph::dot::Dot;

fn main() {

    let mut graph = Graph::new_undirected();
    let na = graph.add_node("a");
    let nb = graph.add_node("b");
    let nc = graph.add_node("c");
    let nd = graph.add_node("d");
    let ne = graph.add_node("e");
    let nf = graph.add_node("f");

    graph.add_edge(na, nb, 20);
    graph.add_edge(nb, nc, 20);
    graph.add_edge(nc, nd, 10);
    graph.add_edge(na, ne, 10);
    graph.add_edge(nb, ne, 10);
    graph.add_edge(nb, nf, 30);
    graph.add_edge(nd, nf, 10);
    graph.add_edge(ne, nf, 20);

    let mut f = fs::File::create("graph.dot").unwrap();
    let dot_str = format!("{:?}", Dot::with_config(&graph, &[]));
    f.write(dot_str.as_bytes()).unwrap();

    let path = algo::dijkstra(
        &graph,
        na,
         Some(nd),
        |e| *e.weight()
    );
    println!("result: {:?}", path);
}
