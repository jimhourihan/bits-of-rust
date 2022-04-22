#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//use petgraph::algo::{dijkstra, min_spanning_tree};
//use petgraph::data::FromElements;
//use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, DiGraph};
use std::env;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

    
struct<'a> Node {
    name: &'a str,
}

struct Top {
    graph: DiGraph::<Node, ()>,
}

impl Node {
    fn new (name: String) -> Node {
        Node {name: name}
    }
}

fn main() {
    // Create an undirected graph with `i32` nodes and edges with `()` associated data.

    let graph = DiGraph::<Node, ()>::new();
}
