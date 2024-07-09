#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_lifetimes)]
#![allow(dead_code)]

use std::{env, marker};
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::{RefCell,Ref,RefMut};
use std::ops::{Add,Sub,Mul,Div};
use crate::value::Value;
use crate::context::{Symbol,SymbolType,Context};
use crate::ltree::{LTerm};

#[derive(Debug,Clone)]
pub struct ExprTree {
    pub storage: Vec<ETerm>,
    pub context: Context,
}

#[derive(Debug,Clone,Copy,Default)]
pub struct ETermID(u32);

#[derive(Debug,Clone)]
pub enum ETerm {
    ValueTerm(Value),
    DerefTerm(Rc<Symbol>),
    ApplicationTerm {
	symbol: Rc<Symbol>,
	args: Vec<ETermID>,
    },
}

pub struct EvalContext<'a> {
    context: &'a Context,
}

#[derive(Debug,Clone)]
pub struct EvalFunc(fn(&EvalContext,&[ETerm],&ETerm,&[ETermID])->Value);

impl ETerm {
    fn eval (&self, context: &EvalContext, terms: &[ETerm]) -> Value {
	match self {
	    ETerm::ApplicationTerm { ref symbol, ref args, .. } => {
		match symbol.symbol_type {
		    SymbolType::Function(ref f) => (f.0)(context, terms, self, args),
		    SymbolType::Variable => Value::Unit,
		    _ => Value::Unit
		}
	    }
	    ETerm::ValueTerm(v) => v.clone(),
	    ETerm::DerefTerm(ref s) => Value::Error, // TODO
	    //_ => Value::Unit
	}
    }

    fn eval_arg (&self, context: &EvalContext, terms: &[ETerm], arg_id: ETermID) -> Value {
	let arg_term = terms.get(arg_id.index()).unwrap();
	arg_term.eval(context, terms)
    }
}

impl ETermID {
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

    pub fn get_node (&self, id: ETermID) -> &ETerm {
	self.storage.get(id.index()).unwrap()
    }

    pub fn next_id (&self) -> ETermID {
	ETermID(self.storage.len() as u32)
    }

    pub fn new_deref_term (&mut self, symbol: Rc<Symbol>) -> ETermID {
	let id = self.next_id();
	self.storage.push(ETerm::DerefTerm(symbol));
	id
    }

    pub fn new_symbol_term (&mut self, symbol: Rc<Symbol>, args: Vec<ETermID>) -> ETermID {
	let id = self.next_id();
	self.storage.push(ETerm::ApplicationTerm { symbol, args });
	id
    }

    pub fn new_value_term (&mut self, value: Value) -> ETermID {
	let id = self.next_id();
	self.storage.push(ETerm::ValueTerm(value));
	id
    }

    pub fn eval_root (&self) -> Value {
	if self.storage.len() > 0 {
	    let context = EvalContext{ context: &self.context };
	    self.storage.last().unwrap().eval(&context, self.storage.as_slice())
	} else {
	    Value::Unit
	}
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
	fn $name (context: &EvalContext, terms: &[ETerm], term: &ETerm, args: &[ETermID]) -> Value {
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
	fn $name (context: &EvalContext, terms: &[ETerm], term: &ETerm, args: &[ETermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		Value::Int64($op a)
	    } else {
		Value::Error
	    }
	}
    };
    ($name:ident, $func:ident, 1) => {
    	fn $name (context: &EvalContext, terms: &[ETerm], term: &ETerm, args: &[ETermID]) -> Value {
	    if let Value::Int64(a) = term.eval_arg(context, terms, args[0]) {
		Value::Int64($func(a))
	    } else {
		Value::Error
	    }
	}
    };
    ($name:ident, $func:ident, 2) => {
    	fn $name (context: &EvalContext, terms: &[ETerm], term: &ETerm, args: &[ETermID]) -> Value {
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

pub fn init_functions (context: &mut Context) {
    context.add_symbol("add".to_string(), SymbolType::Function(EvalFunc(add)));
    context.add_symbol("sub".to_string(), SymbolType::Function(EvalFunc(sub)));
    context.add_symbol("mul".to_string(), SymbolType::Function(EvalFunc(mult)));
    context.add_symbol("div".to_string(), SymbolType::Function(EvalFunc(div)));
    context.add_symbol("neg".to_string(), SymbolType::Function(EvalFunc(neg)));
    context.add_symbol("pow".to_string(), SymbolType::Function(EvalFunc(exp)));
}

pub fn convert (lterm: Box<LTerm>) -> Result<ExprTree,String> {

    fn convert_rec (lterm: &LTerm, etree: &mut ExprTree) -> Result<ETermID,String> {
	match *lterm {
	    LTerm::ValueTerm(ref x) => Result::Ok(etree.new_value_term(x.clone())),
	    LTerm::DerefTerm(ref name) => {
		let sym = etree.context.find_or_add_symbol(&name, SymbolType::Variable);
		Result::Ok(etree.new_deref_term(sym))
	    },
	    LTerm::ApplicationTerm { ref symbol, ref args } => {
		if let Some(sym) = etree.context.find_symbol(&symbol) {
		    let new_args = args.iter()
			.map(|t| convert_rec(&t, etree))
			.collect::<Result<Vec<ETermID>,String>>()?;
		    Result::Ok(etree.new_symbol_term(sym, new_args))
		} else {
		    // couldn't find the function
		    Result::Err(format!("Can't find function {symbol}"))
		}
	    }
	}
    }

    let mut etree = ExprTree::new();
    init_functions(&mut etree.context);
    if let Err(e) = convert_rec(&*lterm, &mut etree) {
	Result::Err(e)
    } else {
	Result::Ok(etree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testoo () {
	let mut etree = ExprTree::new();
	init_functions(&mut etree.context);
	let one_id = etree.new_value_term(Value::Int64(1));
	let two_id = etree.new_value_term(Value::Int64(2));
	let three_id = etree.new_value_term(Value::Int64(3));
	let plus_sym = etree.context.find_symbol("add").unwrap();
	let mult_sym = etree.context.find_symbol("mul").unwrap();
	let added_id = etree.new_symbol_term(plus_sym, vec![one_id, two_id]);
	let multed_id = etree.new_symbol_term(mult_sym, vec![added_id, three_id]);

	let v = etree.eval_root();
	dbg!(v);
    }
}
