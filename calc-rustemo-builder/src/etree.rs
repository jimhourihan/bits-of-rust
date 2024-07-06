#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]

use std::{env, marker};
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{RefCell,Ref,RefMut};
use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug,Clone)]
/// The Context (Γ) holds the symbol table.
pub struct Context {
    pub symbols: HashMap<String,Rc<Symbol>>,
}

#[derive(Debug,Clone)]
pub struct ExprTree {
    pub storage: Vec<Term>,
    pub context: Context,
}

#[derive(Debug,Clone,Copy,Default)]
pub struct TermID(u32);

#[derive(Debug,Clone,Default)]
pub enum Value {
    #[default]
    Unit,
    Error,
    Int64(i64),
}


#[derive(Debug,Clone)]
pub enum Term {
    SymbolTerm {
	symbol: Rc<Symbol>,
	args: Vec<TermID>,
    },
    ValueTerm(Value)
}

pub struct EvalContext {}

#[derive(Debug,Clone)]
pub struct EvalFunc(fn(&EvalContext,&[Term],&Term,&[TermID])->Value);

#[derive(Debug,Clone)]
pub enum SymbolType {
    Symbol,
    Function(EvalFunc),
    Variable,
}

#[derive(Debug,Clone)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
}

impl Term {
    fn eval (&self, context: &EvalContext, terms: &[Term]) -> Value {
	match self {
	    Term::SymbolTerm { ref symbol, ref args, .. } => {
		match symbol.symbol_type {
		    SymbolType::Function(ref f) => (f.0)(context, terms, self, args),
		    SymbolType::Variable => Value::Unit,
		    _ => Value::Unit
		}
	    }
	    Term::ValueTerm(v) => v.clone(),
	    _ => Value::Unit
	}
    }

    fn eval_arg (&self, context: &EvalContext, terms: &[Term], arg_id: TermID) -> Value {
	let arg_term = terms.get(arg_id.index()).unwrap();
	arg_term.eval(context, terms)
    }
}

impl TermID {
    pub fn new (id: u32) -> Self { Self(id) }
    pub fn index (&self) -> usize { self.0 as usize }
}

impl ExprTree {
    pub fn new () -> Self {
	Self {
	    storage: Vec::new(),
	    context: Context::new()
	}
    }

    pub fn get_node (&self, id: TermID) -> &Term {
	self.storage.get(id.index()).unwrap()
    }

    pub fn next_id (&self) -> TermID {
	TermID(self.storage.len() as u32)
    }

    pub fn new_symbol_term (&mut self, symbol: Rc<Symbol>, args: Vec<TermID>) -> TermID {
	let id = self.next_id();
	self.storage.push(Term::SymbolTerm { symbol, args });
	id
    }

    pub fn new_value_term (&mut self, value: Value) -> TermID {
	let id = self.next_id();
	self.storage.push(Term::ValueTerm(value));
	id
    }

    pub fn eval_root (&self) -> Value {
	if self.storage.len() > 0 {
	    let context = EvalContext{};
	    self.storage.last().unwrap().eval(&context, self.storage.as_slice())
	} else {
	    Value::Unit
	}
    }
}

impl Symbol {
    pub fn new (name: &str, symbol_type: SymbolType) -> Self {
	Self { name: name.to_string(), symbol_type }
    }
}

/// This macro allows for three cases:
///
///	1. `expr_func(NAME, op=BINOP)` -- declares a binary operator
///	2. `expr_func(NAME, uop=UOP)`  -- declares a unary operator
///	3. `expr_func(NAME, FUNC, N)`  -- declares an N argument function
///
/// # Example:
///	expr_func!(add_eval_func, op=+)
///	expr_func!(uminus_eval_func, uop=-)
///	expr_func!(my_eval_func, do_something, 1)
///
macro_rules! expr_func {
    ($name:ident,op = $op:tt) => {
	fn $name (context: &EvalContext, terms: &[Term], term: &Term, args: &[TermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		if let Value::Int64(b) = term.eval_arg(context, terms, args[1]) {
		    Value::Int64(a $op b)
		} else {
		    Value::Error
		}
	    } else {
		Value::Error
	    }
	}
    };
    ($name:ident,uop = $op:tt) => {
	fn $name (context: &EvalContext, terms: &[Term], term: &Term, args: &[TermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		Value::Int64($op a)
	    } else {
		Value::Error
	    }
	}
    };
    ($name:ident, $func:ident, 1) => {
    	fn $name (context: &EvalContext, terms: &[Term], term: &Term, args: &[TermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		Value::Int64($func(a))
	    } else {
		Value::Error
	    }
	}
    };
    ($name:ident, $func:ident, 2) => {
    	fn $name (context: &EvalContext, terms: &[Term], term: &Term, args: &[TermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		if let Value::Int64(b) = term.eval_arg(context, terms, args[1]) {
		    Value::Int64($func(a, b))
		} else {
		    Value::Error
		}
	    } else {
		Value::Error
	    }
	}
    };
}

fn unoop (a: i64) -> i64 { a }
fn myexp (a: i64, b: i64) -> i64 { a.pow(b.abs() as u32) }

expr_func!(add, op=+);
expr_func!(sub, op=-);
expr_func!(mult, op=*);
expr_func!(div, op=/);
expr_func!(neg, uop=-);
expr_func!(posi, unoop, 1);
expr_func!(exp, myexp, 2);

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

    pub fn init_functions (&mut self) {
    	self.add_symbol("add".to_string(), SymbolType::Function(EvalFunc(add)));
	self.add_symbol("sub".to_string(), SymbolType::Function(EvalFunc(sub)));
	self.add_symbol("mul".to_string(), SymbolType::Function(EvalFunc(mult)));
	self.add_symbol("div".to_string(), SymbolType::Function(EvalFunc(div)));
	self.add_symbol("neg".to_string(), SymbolType::Function(EvalFunc(neg)));
	self.add_symbol("pow".to_string(), SymbolType::Function(EvalFunc(exp)));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testoo () {
	let mut etree = ExprTree::new();
	etree.context.init_functions();
	etree.context.add_symbol("x".to_string(), SymbolType::Variable);
	let one_id = etree.new_value_term(Value::Int64((1)));
	let two_id = etree.new_value_term(Value::Int64((2)));
	let three_id = etree.new_value_term(Value::Int64((3)));
	let plus_sym = etree.context.find_symbol("add".to_string()).unwrap();
	let mult_sym = etree.context.find_symbol("mul".to_string()).unwrap();
	let added_id = etree.new_symbol_term(plus_sym, vec![one_id, two_id]);
	let multed_id = etree.new_symbol_term(mult_sym, vec![added_id, three_id]);
	//let x_sym = etree.context.get_symbol("x".to_string()).clone();
	//let x_id = etree.new_symbol_term(x_sym, vec![]);
	//let plus2_sym = etree.context.get_symbol("+".to_string()).clone();
	//let added2_id = etree.new_symbol_term(plus2_sym, vec![added_id, x_id]);

	let v = etree.eval_root();
	dbg!(v);
	//dbg!(&etree);
    }
}
