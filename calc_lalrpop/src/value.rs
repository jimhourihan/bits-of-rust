#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]

use std::rc::Rc;
use crate::context::{Symbol};
use crate::ltree::{LTerm};

#[derive(Debug,Clone,Default)]
pub enum Value {
    #[default]
    Unit,
    Error,
    QuotedExpr(Box<LTerm>),
    Int64(i64),
}
