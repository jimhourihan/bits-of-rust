#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]

use std::env;
use std::rc::Rc;
use std::collections::HashMap;
use typed_arena::Arena;
use std::cell::{RefCell,Ref,RefMut};

#[derive(Debug,Clone,Default)]
pub struct ETree {
    pub storage: Vec<ENode>,
    pub context: Context,
}

#[derive(Debug,Clone,Default)]
pub struct ENodeID(u32);
pub type ENodeIDVec = Vec<ENodeID>;

#[derive(Debug,Clone)]
pub struct ENode{
    symbol: SymbolRef,
    children: Vec<ENodeID>
}

#[derive(Debug,Clone,Default)]
pub enum SymbolType {
    #[default]
    Symbol,
    Type,
    Function,
    Variable,
    Parameter,
}

#[derive(Debug,Clone,Default)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
}

type SymbolRef = Rc<RefCell<Symbol>>;

#[derive(Debug,Clone,Default)]
pub struct Context {
    symbols: HashMap<String,SymbolRef>
}

pub struct CompilationUnit {
    symbol_arena: Arena<Symbol>,
    root_context: Context,
}

impl CompilationUnit {
    pub fn new () -> Self {
	Self { symbol_arena: Arena::new(), root_context: Context::default() }
    }

    pub fn new_symbol (&self, name: String, symbol_type: SymbolType) -> &mut Symbol {
	self.symbol_arena.alloc(Symbol::new(name, symbol_type))
    }
}

impl ETree {
    pub fn new () -> Self {
	Self { storage: Vec::new(), context: Context::default() }
    }

    pub fn new_enode (&mut self, symbol: SymbolRef, children: Vec<ENodeID>) -> ENodeID {
	let id = self.storage.len() as u32;
	self.storage.push(ENode { symbol, children });
	ENodeID ( id )
    }
}

impl Symbol {
    pub fn new (name: String, symbol_type: SymbolType) -> Self {
	Self { name, symbol_type }
    }
}

impl Context {
    pub fn new () -> Self {
	Self { symbols: HashMap::new() }
    }

    pub fn insert (&mut self, symbol: Symbol) {
	let _ = self.symbols.insert(symbol.name.clone(), Rc::new(RefCell::new(symbol)));
    }

    pub fn get_symbolref (&self, name: &str) -> Option<&SymbolRef> {
	self.symbols.get(name)
    }
    
    pub fn get (&self, name: &str) -> Option<Ref<Symbol>> {
	self.get_symbolref(name).map(|s| s.borrow())
    }

    pub fn get_mut (&mut self, name: &str) -> Option<RefMut<Symbol>> {
	self.get_symbolref(name).map(|s| s.borrow_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foo () {
	let mut etree = ETree::new();
	let symbol = Symbol::new("Add".to_string(), SymbolType::Symbol);
	etree.context.insert(symbol);
	//let t = etree.context.get("Add");
	//dbg!(t);
	dbg!(etree);
    }
}
