mod ast;
mod code;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::io::{stdin, stdout};

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    let input = stdin();
    repl::start(input.lock(), stdout().lock())
}
