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

    // pub fn pop1 (&mut self) -> Option<TermID> { self.stack.pop() }

    // pub fn pop2 (&mut self) -> Option<(TermID,TermID)> {
    // 	let a = self.stack.pop().unwrap();
    // 	let b = self.stack.pop().unwrap();
    // 	Option::Some((a,b))
    // }

    // pub fn pop3 (&mut self) -> Option<(TermID,TermID,TermID)> {
    // 	let a = self.stack.pop().unwrap();
    // 	let b = self.stack.pop().unwrap();
    // 	let c = self.stack.pop().unwrap();
    // 	Option::Some((a,b,c))
    // }

    pub fn reduce_unary_op (&mut self, op_name: &str) {
	let op_sym = self.etree.context.get_symbol(op_name.to_string()).clone();
	let a_id = self.stack.pop().unwrap();
	let func_id = self.etree.new_symbol_term(op_sym, vec![a_id]);
	self.stack.push(func_id);
    }

    pub fn reduce_binary_op (&mut self, op_name: &str) {
	let op_sym = self.etree.context.get_symbol(op_name.to_string()).clone();
	let b_id = self.stack.pop().unwrap();
	let a_id = self.stack.pop().unwrap();
	let func_id = self.etree.new_symbol_term(op_sym, vec![a_id, b_id]);
	self.stack.push(func_id);
    }

    pub fn shift_value (&mut self, val: Value) {
	self.push_next();
	self.etree.new_value_term(val);
    }

    pub fn shift_symbol (&mut self, name: String) {
	let symbol = Rc::new(Symbol::new(name, SymbolType::Symbol));
	let args: Vec<TermID> = Vec::new();
	self.push_next();
	self.etree.new_symbol_term(symbol, args);
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
	dbg!(&token);
        match token.kind {
	    TokenKind::Number => self.shift_value(Value::Int64(token.value.parse().unwrap())),
	    TokenKind::Symbol => self.shift_symbol(token.value.to_string()),
	    _ => ()
	}
    }

    fn reduce_action(&mut self,
		     _context: &mut Context<'i>,
		     prod: ProdKind,
		     _prod_len: usize)
    {
	dbg!(prod);
        let res = match prod {
	    ProdKind::ExprExpr => (),
	    ProdKind::BinaryExprUnary => (),
	    ProdKind::BinaryExprAdd => self.reduce_binary_op("+"),
	    ProdKind::BinaryExprSub => self.reduce_binary_op("-"),
	    ProdKind::BinaryExprMult => self.reduce_binary_op("*"),
	    ProdKind::BinaryExprDiv => self.reduce_binary_op("/"),
	    ProdKind::BinaryExprExp => self.reduce_binary_op("^"),
	    ProdKind::UnaryExprLiteral => (),
	    ProdKind::UnaryExprParen => (),
	    ProdKind::UnaryExprUminus => self.reduce_unary_op("-"),
	    ProdKind::UnaryExprUplus => self.reduce_unary_op("+"), // fixme
	    ProdKind::LiteralNumber => (),
	    ProdKind::LiteralSymbol => (),
	    _ => ()
	};
    }
}
