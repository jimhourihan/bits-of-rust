
#[derive(Clone)]
struct Perm;

impl Perm {
    fn new () -> Self {
        Self {}
    }
}


fn main() {
    let a = Perm::new();
    let b = a;
    let c = b.clone();
}
