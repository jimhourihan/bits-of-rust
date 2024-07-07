
#[derive(Debug,Clone,Default)]
pub enum Value {
    #[default]
    Unit,
    Error,
    Int64(i64),
}
