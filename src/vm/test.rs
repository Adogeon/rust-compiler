use super::*;
use crate::ast::Program;
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

struct VmTestCase {
    input: &'static str,
    expected: Object,
}

#[test]
fn test_integer_arithmetic() -> Result<(), String> {
    let test_cs = vec![
        VmTestCase {
            input: "1",
            expected: Object::INTEGER(1),
        },
        VmTestCase {
            input: "2",
            expected: Object::INTEGER(2),
        },
        VmTestCase {
            input: "1 + 2",
            expected: Object::INTEGER(3),
        },
    ];

    run_vm_tests(test_cs)
}

fn run_vm_tests(tc: Vec<VmTestCase>) -> Result<(), String> {
    for test in tc {
        let prog = parse(test.input);
        let mut comp = Compiler::new();
        comp.compile(prog.into())?;
        let mut vm = VM::new(comp.bytecode());
        vm.run()?;
        let stack_ele = vm.stack_top();
        test_exepected_object(test.expected, stack_ele.unwrap())?;
    }

    Ok(())
}

fn test_exepected_object(expected: Object, actual: Object) -> Result<(), String> {
    match expected {
        Object::INTEGER(int) => {
            test_integer_object(&actual, int)?;
        }
        _ => return Err(String::from("expected type isn't implement")),
    };

    Ok(())
}

fn parse(input: &str) -> Program {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    p.parse_program()
        .unwrap_or_else(|_| Program { statements: vec![] })
}

pub fn test_integer_object(obj: &Object, expected: i64) -> Result<(), String> {
    if let Object::INTEGER(val) = obj {
        if *val == expected {
            Ok(())
        } else {
            Err(format!(
                "Object value is not equal to expected:{expected}, got {}",
                val
            ))
        }
    } else {
        Err(String::from("object is not an Integer Object"))
    }
}
