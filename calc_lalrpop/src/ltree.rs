#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]

use std::{env, marker};
use std::rc::Rc;
use crate::context::{Symbol,Context};
use crate::value::Value;

#[derive(Debug,Clone)]
/// Initial AST structure compiled by parser
pub enum LTerm {
    ValueTerm(Value),
    DerefTerm(String),
    ApplicationTerm {
        symbol: String,
        args: Vec<LTerm>
    },
}

// Parser noise reduction

pub fn lterm_app (sym_name: &str, args: Vec<LTerm>) -> Box<LTerm> {
    Box::new(LTerm::ApplicationTerm { symbol: sym_name.to_string(), args })
}

pub fn lterm_sapp (symbol: String, args: Vec<LTerm>) -> Box<LTerm> {
    Box::new(LTerm::ApplicationTerm { symbol, args })
}

pub fn lterm_value (v: Value) -> Box<LTerm> {
    Box::new(LTerm::ValueTerm(v))
}

pub fn lterm_symbol (s: String) -> Box<LTerm> {
    Box::new(LTerm::DerefTerm(s))
}


