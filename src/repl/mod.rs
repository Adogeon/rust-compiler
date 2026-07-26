use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::environment::Environment;
use crate::object::Object;
use crate::parser::{ParseError, Parser};
use crate::vm::VM;
use std::io::{self, BufRead, Write};

pub fn start(mut in_handler: io::StdinLock, mut out_handler: io::StdoutLock) {
    let mut buffer = String::new();
    let env = Environment::new();
    loop {
        write!(out_handler, ">>").unwrap();
        out_handler.flush().unwrap();

        buffer.clear();
        if let Err(e) = in_handler.read_line(&mut buffer) {
            writeln!(out_handler, "Error reading line: {}", e).unwrap();
            break;
        }

        let l = Lexer::new(&buffer);
        let mut p = Parser::new(l);
        let program = match p.parse_program() {
            Ok(prog) => prog,
            Err(err) => {
                print_parser_errors(&mut out_handler, err);
                continue;
            }
        };

        let mut comp = Compiler::new();
        let _ = comp.compile(program.into());
        let mut machine = VM::new(comp.bytecode());
        let _ = machine.run();
        let stack_top = machine.last_popped_stack_elm();
        writeln!(out_handler, "{}", stack_top.inspect()).unwrap();
    }
}

fn print_parser_errors(out_handler: &mut io::StdoutLock, error: ParseError) {
    writeln!(out_handler, "{error}").unwrap();
}
