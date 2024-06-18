use etree::{ETree, CompilationUnit};
use rustemo::Parser;
use std::io;
use std::result::Result;
// Use the generated parser
use crate::calculator::{CalculatorParser};
use calculator_actions::{Expr};
mod etree;

// Include generated modules
#[rustfmt::skip]
mod calculator;
#[allow(unused)]
#[rustfmt::skip]
//use calculator::{Expr};
mod calculator_actions;

fn main() {
    let mut expression = String::new();

    // Read the line from the input
    println!("Expression:");
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    // Parse the line and get the result.
    // Expr is ultimately aliased to Vec<Vec<ConditionalExpr>>
    let result_parse: Result<Expr,_> = CalculatorParser::new().parse(&expression);
    println!("{:#?}", result_parse);
}

