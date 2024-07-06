use std::env;
use rustemo::{LRContext,LRBuilder,Builder,Token};
use crate::{
    calculator::{State,TokenKind,ProdKind},
    etree::{ExprTree,Value,Term,SymbolType,Symbol,TermID}
};
use std::rc::Rc;

pub type Context<'i> = LRContext<'i, str, State, TokenKind>;

pub struct ETreeBuilder<'a> {
    pub etree: &'a mut ExprTree,
    pub stack: Vec<TermID>,
}

impl<'a> ETreeBuilder<'a> {
    pub fn new(state: &'a mut ExprTree) -> Self {
        Self { etree: state, stack: Vec::new() }
    }

    pub fn push_next (&mut self) {
	self.stack.push(self.etree.next_id());
    }

    pub fn reduce_func1 (&mut self, func_name: &str) {
	if let Some(sym) = self.etree.context.find_symbol(func_name) {
	    let a_id = self.stack.pop().unwrap();
	    let func_id = self.etree.new_symbol_term(sym, vec![a_id]);
	    self.stack.push(func_id);
	} 
    }

    pub fn reduce_func2 (&mut self, func_name: &str) {
	let sym = self.etree.context.find_symbol(func_name).unwrap(); // FIXME
	let b_id = self.stack.pop().unwrap();
	let a_id = self.stack.pop().unwrap();
	let func_id = self.etree.new_symbol_term(sym, vec![a_id, b_id]);
	self.stack.push(func_id);
    }

    pub fn shift_value (&mut self, val: Value) {
	self.push_next();
	self.etree.new_value_term(val);
    }

    pub fn shift_symbol (&mut self, name: &str) {
	if let Some(symbol) = self.etree.context.find_symbol(name) {
	    let args: Vec<TermID> = Vec::new();
	    self.push_next();
	    self.etree.new_symbol_term(symbol, args);
	} else {
	    let symbol = Rc::new(Symbol::new(name, SymbolType::Symbol));
	    let args: Vec<TermID> = Vec::new();
	    self.push_next();
	    self.etree.new_symbol_term(symbol, args);
	}
    }

}

impl<'a> Builder for ETreeBuilder<'a> {
    type Output = ();
    fn get_result(&mut self) -> Self::Output {}
}

impl<'i> LRBuilder<'i, str, Context<'i>, State, ProdKind, TokenKind>
    for ETreeBuilder<'i>
{
    fn shift_action(&mut self,
		    _context: &mut Context<'i>,
		    token: Token<'i, str, TokenKind>)
    {
        match token.kind {
	    TokenKind::Number => self.shift_value(Value::Int64(token.value.parse().unwrap())),
	    TokenKind::Symbol => self.shift_symbol(token.value),
	    _ => ()
	}
    }

    fn reduce_action(&mut self,
		     _context: &mut Context<'i>,
		     prod: ProdKind,
		     _prod_len: usize)
    {
        let res = match prod {
	    ProdKind::ExprExpr => (),
	    ProdKind::BinaryExprUnary => (),
	    ProdKind::BinaryExprAdd => self.reduce_func2("add"),
	    ProdKind::BinaryExprSub => self.reduce_func2("sub"),
	    ProdKind::BinaryExprMult => self.reduce_func2("mul"),
	    ProdKind::BinaryExprDiv => self.reduce_func2("div"),
	    ProdKind::BinaryExprExp => self.reduce_func2("pow"),
	    ProdKind::UnaryExprPrimary => (),
	    ProdKind::UnaryExprParen => (),
	    ProdKind::UnaryExprUminus => self.reduce_func1("neg"),
	    ProdKind::LiteralNumber => (),
	    ProdKind::PrimaryExprSymbol => (),
	    ProdKind::PrimaryExprFunCall1 => (),
	    ProdKind::PrimaryExprFunCall2 => (),
	    ProdKind::LiteralNumber => (),
	    _ => ()
	};
    }
}
