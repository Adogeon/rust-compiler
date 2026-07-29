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
        VmTestCase {
            input: "1 - 2",
            expected: Object::INTEGER(-1),
        },
        VmTestCase {
            input: "1 * 2",
            expected: Object::INTEGER(2),
        },
        VmTestCase {
            input: "4 / 2",
            expected: Object::INTEGER(2),
        },
        VmTestCase {
            input: "50 / 2 * 2 + 10 - 5",
            expected: Object::INTEGER(55),
        },
        VmTestCase {
            input: "2 * 2 * 2 * 2 * 2",
            expected: Object::INTEGER(32),
        },
        VmTestCase {
            input: "5 * 2 + 10",
            expected: Object::INTEGER(20),
        },
        VmTestCase {
            input: "5 + 2 * 10",
            expected: Object::INTEGER(25),
        },
        VmTestCase {
            input: "5 * (2 + 10)",
            expected: Object::INTEGER(60),
        },
    ];

    run_vm_tests(test_cs)
}

#[test]
fn test_boolean_expression() -> Result<(), String> {
    let tc = vec![
        VmTestCase {
            input: "true",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "false",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 < 2",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "1 > 2",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 < 1",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 > 1",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 == 1",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "1 != 1",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 == 2",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "1 != 2",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "true == true",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "false == false",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "true == false",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "true != false",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "false != true",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "(1 < 2) == true",
            expected: Object::BOOLEAN(true),
        },
        VmTestCase {
            input: "(1 < 2) == false",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "(1 > 2) == true",
            expected: Object::BOOLEAN(false),
        },
        VmTestCase {
            input: "(1 > 2) == false",
            expected: Object::BOOLEAN(true),
        },
    ];
    run_vm_tests(tc)
}

fn run_vm_tests(tc: Vec<VmTestCase>) -> Result<(), String> {
    for test in tc {
        let prog = parse(test.input);
        let mut comp = Compiler::new();
        comp.compile(prog.into())?;
        let mut vm = VM::new(comp.bytecode());
        vm.run()?;
        let stack_ele = vm.last_popped_stack_elm();
        test_exepected_object(test.expected, stack_ele)?;
    }

    Ok(())
}

fn test_exepected_object(expected: Object, actual: Object) -> Result<(), String> {
    match expected {
        Object::INTEGER(int) => {
            test_integer_object(&actual, int)?;
        }
        Object::BOOLEAN(val) => {
            test_boolean_object(&actual, val)?;
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

fn test_boolean_object(input: &Object, expected: bool) -> Result<(), String> {
    if let Object::BOOLEAN(s) = input {
        if *s == expected {
            Ok(())
        } else {
            Err(format!("expected {expected}, got={s}"))
        }
    } else {
        Err(format!("input is not a Boolean Object"))
    }
}
