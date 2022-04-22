#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::env;
use std::collections::HashMap;

pub type ID = u32;

//----------------------------------------------------------------------

pub struct Edge {
    pub id: ID,
    pub from_node: ID,
    pub to_node: ID
}

pub type EdgeVec = Vec<ID>;

impl Edge {
    fn new (id: ID, from_node: ID, to_node: ID) -> Edge {
        Edge {
            id: id,
            from_node: from_node,
            to_node: to_node
        }
    }
}

//----------------------------------------------------------------------

pub struct Node {
    pub id: ID,
    pub name: String,
    pub ins: EdgeVec,
    pub outs: EdgeVec
}

pub type NodeMap = HashMap<ID,Node>;
pub type EdgeMap = HashMap<ID,Edge>;

impl Node {
    fn new (id: ID, name: String) -> Node {
        Node {
            id: id,
            name: name,
            ins: EdgeVec::new(),
            outs: EdgeVec::new()
        }
    }
}

//----------------------------------------------------------------------

pub struct Graph {
    node_map: NodeMap,
    edge_map: EdgeMap,
    current_id: ID,
}

impl Graph {
    fn new () -> Graph {
        Graph {
            current_id: 1,
            node_map: NodeMap::new(),
            edge_map: EdgeMap::new()
        }
    }

    fn new_node (&mut self, name: String) -> ID {
        let id = self.current_id;
        self.node_map.insert(id, Node::new(id, name));
        self.current_id += 1;
        id
    }

    fn new_edge (&mut self, from_node: ID, to_node: ID) -> Option<ID> {
        use std::collections::hash_map::Entry::Occupied;
        if self.node_map.contains_key(&from_node) && self.node_map.contains_key(&to_node) {
            if from_node != to_node {
                let id = self.current_id;
                self.edge_map.insert(id, Edge::new(id, from_node, to_node));
                self.current_id += 1;

                // Update node EdgeVecs. Should real code use something
                // other than the unwrap? Seems like this can't fail.

                self.node_map.get_mut(&from_node).unwrap().outs.push(id);
                self.node_map.get_mut(&to_node).unwrap().ins.push(id);

                Some(id)
            } else { None }
        } else { None }
    }
}

//----------------------------------------------------------------------

fn main() {
    let mut graph = Graph::new();
    let mut ids = Vec::<ID>::new();
    for name in ["a", "b", "c", "d", "e"] {
        println!("node: {}", name);
        ids.push( graph.new_node(name.to_string()) );
    }

    for i in 0..ids.len()-1 {
        println!("edge: {} -> {}", ids[i], ids[i+1]);
        graph.new_edge(ids[i], ids[i+1]);
    }
}
