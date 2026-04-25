
use gc_arena::{self, Arena, Collect, Gc, Rootable, lock::RefLock};

#[derive(Debug,Collect)]
#[collect(no_drop)]
struct GraphArenaRoot<'gc> {
    data: i32,
    objects: Vec<NodePtr<'gc>>,
}

impl<'gc> GraphArenaRoot<'gc> {
    fn new () -> Self {
        GraphArenaRoot {
            data: 0,
            objects: Vec::new()
        }
    }
}

#[derive(Debug,Collect)]
#[collect(no_drop)]
struct Node<'gc> {
    name: String,
    inputs: Vec<NodePtr<'gc>>,
    outputs: Vec<NodePtr<'gc>>,
}

type NodePtr<'gc> = Gc<'gc,RefLock<Node<'gc>>>;

impl<'gc> Node<'gc> {
    fn new (name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

type GraphArena = Arena<Rootable![GraphArenaRoot<'_>]>;

fn main() {
    let mut arena = GraphArena::new(|_mc| { // mc is mutation context (not using here)
        GraphArenaRoot::new()
    });

    arena.mutate_root(|mc, root| {
        let node1 = Gc::new(mc, RefLock::new(Node::new("node1")));
        root.objects.push(node1);
    });

    arena.mutate_root(|mc, root| {
        let node2 = Gc::new(mc,RefLock::new(Node::new("node2")));
        let node3 = Gc::new(mc,RefLock::new(Node::new("node3")));
        let mut node1 = root.objects.first().unwrap().borrow_mut(mc);
        node1.inputs.push(node2);
        node1.inputs.push(node3);
    });

    arena.mutate_root(|mc, root| {
        let node4 = Gc::new(mc,RefLock::new(Node::new("node4")));
        let node5 = Gc::new(mc,RefLock::new(Node::new("node5")));
        let node1 = root.objects.first().unwrap().borrow();
        let mut node2 = node1.inputs.first().unwrap().borrow_mut(mc);
        node2.inputs.push(node4);
        node2.inputs.push(node5);

        // should look like this:
        //
        //                   ╭──node4
        //        ╭──node2───┤
        // node1╶─┤          ╰──node5
        //        ╰──node3
        //
    });

    arena.mutate(|_, root| {
        dbg!(root.objects.first().unwrap().borrow());
    });

    dbg!(std::mem::size_of::<Node>());
    dbg!(std::mem::size_of::<RefLock<Node>>());
}
