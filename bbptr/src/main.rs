use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ptr::NonNull,
    sync::{Arc, RwLock},
};

struct BBInner<T> {
    obj: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct BBPtr<'arena, T> {
    ptr: NonNull<BBInner<T>>,
    _marker: PhantomData<&'arena BBInner<T>>,
}

impl<'arena, T> Clone for BBPtr<'arena, T> {
    fn clone(&self) -> Self { *self }
}

impl<'arena, T> Copy for BBPtr<'arena, T> {}

unsafe impl<'arena, T: Send> Send for BBPtr<'arena, T> {}
unsafe impl<'arena, T: Send + Sync> Sync for BBPtr<'arena, T> {}

impl<'arena, T> BBPtr<'arena, T> {
    pub fn get<'e>(&'e self, _: &'e Access<'arena>) -> &'e T {
        unsafe { &*(*self.ptr.as_ptr()).obj.get() }
    }

    pub fn get_mut<'e>(&'e self, _: &'e mut Access<'arena>) -> &'e mut T {
        unsafe { &mut *(*self.ptr.as_ptr()).obj.get() }
    }
}

pub struct Access<'arena> {
    _marker: PhantomData<fn(&'arena ()) -> &'arena ()>,
}


pub struct Graph<'arena> {
    lock: Arc<RwLock<()>>,
    node_arena: Arena<'arena, Node<'arena>>,
    edge_arena: Arena<'arena, Edge<'arena>>,
    root_node: Option<NodePtr<'arena>>,
    _marker: PhantomData<fn(&'arena ()) -> &'arena ()>,
}

impl<'arena> Graph<'arena> {
    fn new() -> Self {
        Graph {
            lock: Arc::new(RwLock::new(())),
            node_arena: Arena { nodes: Vec::new(), _marker: PhantomData },
            edge_arena: Arena { nodes: Vec::new(), _marker: PhantomData },
            root_node: None,
            _marker: PhantomData
        }
    }

    pub fn edit<F, R>(&'arena mut self, f: F) -> R
    where
        F: for<'a> FnOnce(&'a mut Graph<'a>, Access<'a>) -> R
    {
        let lock = self.lock.clone();
        let _guard = lock.write().unwrap();
        f(self, Access { _marker: PhantomData })
    }

    pub fn traverse<F, R>(&'arena self, f: F) -> R
    where
        F: for<'a> FnOnce(&'a Graph<'a>) -> R
    {
        let lock = self.lock.clone();
        let _guard = lock.read().unwrap();
        f(self)
    }
}

pub struct Arena<'arena, T> {
    nodes: Vec<Box<BBInner<T>>>,
    _marker: PhantomData<&'arena ()>,
}

impl<'arena, T> Arena<'arena, T> {
    pub fn alloc(&mut self, obj: T) -> BBPtr<'arena, T> {
        let mut boxed = Box::new(BBInner { obj: UnsafeCell::new(obj) });
        let ptr = NonNull::from(boxed.as_mut());
        self.nodes.push(boxed);
        BBPtr { ptr, _marker: PhantomData }
    }
}

#[derive(Debug)]
pub struct Node<'arena> {
    pub name: String,
    pub incoming_edges: Vec<EdgePtr<'arena>>,
    pub outgoing_edges: Vec<EdgePtr<'arena>>,
}

impl<'arena> Node<'arena> {
    pub fn new (name: &str) -> Self {
        Self {
            name: name.to_string(),
            incoming_edges: Vec::new(),
            outgoing_edges: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Edge<'arena> {
    pub in_node: NodePtr<'arena>,
    pub out_node: NodePtr<'arena>,
}

impl<'arena> Edge<'arena> {
    pub fn new (
        in_node: NodePtr<'arena>,
        out_node: NodePtr<'arena>
    ) -> Self {
        Self {in_node, out_node,}
    }
}

type NodePtr<'arena> = BBPtr<'arena,Node<'arena>>;
type EdgePtr<'arena> = BBPtr<'arena,Edge<'arena>>;

fn main() {
    let mut g = Graph::new();
    g.edit(|graph, mut access| {
        let n1 = graph.node_arena.alloc(Node::new("n1"));
        let n2 = graph.node_arena.alloc(Node::new("n2"));
        let e1 = graph.edge_arena.alloc(Edge::new(n1, n2));
        graph.root_node = Some(n1);
        n1.get_mut(&mut access).outgoing_edges.push(e1);
        n2.get_mut(&mut access).incoming_edges.push(e1);
    });
}
