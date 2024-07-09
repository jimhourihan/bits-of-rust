#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]
use std::{env, marker};
use std::rc::Rc;
use std::collections::HashMap;
use crate::etree::{EvalFunc};

#[derive(Debug,Clone)]
/// The Context (Γ) holds the symbol table.
pub struct Context {
    pub symbols: HashMap<String,Rc<Symbol>>,
}

#[derive(Debug,Clone)]
pub enum SymbolType {
    Symbol,
    Variable,
    Function(EvalFunc)
}

#[derive(Debug,Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
}

impl Symbol {
    pub fn new (name: &str, symbol_type: SymbolType) -> Self {
	Self { name: name.to_string(), symbol_type }
    }
}

impl Context {
    pub fn new () -> Self {
	Self { symbols: HashMap::default() }
    }

    pub fn add_symbol (&mut self, name: String, symbol_type: SymbolType) {
	let s = Symbol::new(&name, symbol_type);
	self.symbols.insert(name, Rc::new(s));
    }

    /// Returns a clone of the Rc<Symbol> in the Context or None
    pub fn find_symbol (&self, name: &str) -> Option<Rc<Symbol>> {
	if let Some(sym_ref) = self.symbols.get(name) {
	    Some(sym_ref.clone())
	} else {
	    None
	}
    }

    pub fn find_or_add_symbol (&mut self, name: &str, symbol_type: SymbolType) -> Rc<Symbol> {
	if let Some(sym) = self.find_symbol(name) {
	    sym
	} else {
	    self.add_symbol(name.to_string(), symbol_type);
	    self.find_symbol(name).unwrap()
	}
    }
}

