#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]
use std::{env, marker};
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug,Clone)]
/// The Context (Γ) holds the symbol table.
pub struct Context {
    pub symbols: HashMap<String,Rc<Symbol>>,
}

#[derive(Debug,Clone)]
pub enum SymbolType {
    Symbol,
    Variable,
}

#[derive(Debug,Clone)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new (name: &str, symbol_type: SymbolType) -> Self {
	Self { name: name.to_string(), symbol_type }
    }
}

impl Context {
    pub fn new () -> Self { Self {symbols: HashMap::default()} }

    pub fn add_symbol (&mut self, name: String, symbol_type: SymbolType) {
	let s = Symbol::new(&name, symbol_type);
	self.symbols.insert(name, Rc::new(s));
    }

    pub fn find_symbol (&self, name: &str) -> Option<Rc<Symbol>> {
	if let Some(sym_ref) = self.symbols.get(name) {
	    Some(sym_ref.clone())
	} else {
	    None
	}
    }
}

