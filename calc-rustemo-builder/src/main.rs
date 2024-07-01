#![allow(unused)]
use rustemo::{Parser};
use std::io;
use std::result::Result;
use crate::{
    calculator::CalculatorParser,
    etree_builder::{ETreeBuilder},
    etree::{ExprTree},
};

mod calculator;
mod etree;
mod etree_builder;

fn main() {
    let mut expression = String::new();

    // Read the line from the input
    println!("Expression:");
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    let mut etree = ExprTree::new();
    etree.context.init_functions();
    
    let result = CalculatorParser::new(ETreeBuilder::new(&mut etree))
        .parse(&expression);

    let r = etree.eval_root();
    dbg!(r);
}

