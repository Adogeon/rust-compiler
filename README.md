# Monkey Lang Interpreter In Rust

A Rust implementation of the **Monkey programming language** interpreter from Thorsten Ball's excellent book ["Writing An Interpreter In Go"](https://interpreterbook.com/). This project explores compiler design, parsing techniques, and language implementation using Rust's powerful type system and memory safety guarantees.

## Status: WIP

### Implemented

- **Lexer**: Transforms source code into a stream of tokens.
- **Parser**: Builds an Abstract Syntax Tree (AST) from the tokens using a Pratt parser.
- **AST Representation**: Full representation of the Monkey language syntax.
- **Evaluator**: Walks the AST and executes the program.
- **REPL**: Interactive Read-Eval-Print Loop
- **Built-in Functions**: Implementing core language built-ins
- **Array and Hash Support**: Implement collections object via Rust Vector and HashMap crate

### In Progress

- **The Lost Chapter Macro**: Allow user to build macro in Monkey

## Building and Running

### Prerequisites

- Rust and Cargo

### Quick Start

1. Clone the repository
2. Run the REPL with `cargo run`

## Project Goals:

- A hand-on study of language interpreter construction
- Practicing programming in Rust
