#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::env;
use std::collections::HashMap;

// This example demonstrates creating a super simple expression tree which
// can be evaluated at runtime. There are three expression types: constant,
// add, and mult. The tree created can't be edited after built for the sake
// of simplicity.

// All of our evaluations are parameterized by some "context" type. This
// could be as simple as a frame number or time or nothing.

struct Context { frame: i32 }

// Generic node interface for some context which evaluates to some type

pub trait Node<C, T> {
    fn evaluate (&self, context: &C) -> T;
}

// An expression in this case is a generic node which uses the Context
// struct for its context and a float for the result type.

type Expr = Box<dyn Node<Context,f32>>;

// A constant expression is a node which returns some exact value no matter
// what the context is. there are no inputs to it

pub struct FloatConstantNode { pub value: f32 }

impl Node<Context, f32> for FloatConstantNode {
    fn evaluate (&self, context: &Context) -> f32 { self.value }
}

fn new_float_constant (v: f32) -> Expr {
    Box::new( FloatConstantNode { value: v } )
}

// A binary operator (+ or *) has two inputs which are themselves
// expressions. These are recursively evaluated before the op is applied.

pub struct AddNode {
    a: Expr,
    b: Expr
}

fn new_add_expr (a: Expr, b: Expr) -> Expr {
    Box::new( AddNode { a: a, b: b } )
}

impl Node<Context,f32> for AddNode {
    fn evaluate (&self, context: &Context) -> f32 {
        self.a.evaluate(context) + self.b.evaluate(context)
    }
}

pub struct MultNode {
    a: Expr,
    b: Expr
}

fn new_mult_expr (a: Expr, b: Expr) -> Expr {
    Box::new( MultNode { a: a, b: b } )
}

impl Node<Context,f32> for MultNode {
    fn evaluate (&self, context: &Context) -> f32 {
        self.a.evaluate(context) * self.b.evaluate(context)
    }
}

// rust's type "()" is the unit type (only a single value which is also
// called "()"). In practice its used like "void" in C/C++ but makes more
// sense when everything is an expression (meaning it has a type and
// computes to a value).

fn main () -> () {

    // (1+2) * (3+4) -> 21
    let a = new_float_constant(1_f32);
    let b = new_float_constant(2_f32);
    let c = new_float_constant(3_f32);
    let d = new_float_constant(4_f32);
    let add1 = new_add_expr(a, b);
    let add2 = new_add_expr(c, d);
    let mult = new_mult_expr(add1, add2);
    let expr = mult;

    let context = Context { frame: 0 };
    let result = expr.evaluate(&context);

    println!("result -> {}", result);
    return () // this is unnecessary -- its the default if the scope ends
              // in ';'
}
