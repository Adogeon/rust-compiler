use super::*;
use crate::{ast::Program, code, lexer::Lexer, object::Object, parser::Parser};

struct CompilerTestCase {
    input: &'static str,
    expected_constants: Vec<u8>,
    expected_instructions: Vec<code::Instructions>,
}

#[test]
fn test_integer_arithmetic() -> Result<(), String> {
    let test_cases: Vec<CompilerTestCase> = vec![CompilerTestCase {
        input: "1+2",
        expected_constants: vec![1, 2],
        expected_instructions: vec![
            code::make(code::OP_CONSTANT, &[0 as u16]),
            code::make(code::OP_CONSTANT, &[1 as u16]),
        ],
    }];

    run_compiler_test(test_cases)
}

fn run_compiler_test(test_cases: Vec<CompilerTestCase>) -> Result<(), String> {
    for tc in test_cases {
        let program = parse(tc.input);
        let compiler = Compiler::new();
        compiler.compile(program)?;
        let bytecode = compiler.bytecode();
        test_instructions(tc.expected_instructions, bytecode.instruction)?;
        test_constants(tc.expected_constants, bytecode.constants)?;
    }
    Ok(())
}

fn parse(input: &str) -> ast::Program {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    p.parse_program()
        .unwrap_or_else(|_| Program { statements: vec![] })
}

fn test_constants(expected_constants: Vec<u8>, constants: Vec<Object>) -> Result<(), String> {
    todo!()
}

fn test_instructions(
    expected_instructions: Vec<Vec<u8>>,
    instruction: Vec<u8>,
) -> Result<(), String> {
    let concatted = expected_instructions.concat();
    assert_eq!(
        concatted.len(),
        instruction.len(),
        "wrong instruction length; expect {}, got {}",
        instruction.len(),
        concatted.len()
    );

    for (i, ins) in concatted.iter().enumerate() {
        assert_eq!(
            instruction[i], *ins,
            "wrong instruction at {}; expect {}, got {}",
            i, ins, instruction[i]
        )
    }

    Ok(())
}
