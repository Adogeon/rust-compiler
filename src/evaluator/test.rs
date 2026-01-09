use std::collections::HashMap;

use super::*;
use crate::object::hash::HashKey;
use crate::object::Object;
use crate::{lexer::Lexer, parser::Parser};

#[test]
fn test_eval_integer_expression() -> Result<(), String> {
    let test_cases: Vec<(&str, i64)> = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        test_integer_object(&evaluated, expected)?;
    }
    Ok(())
}

fn test_eval(input: &str) -> Result<Object, String> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error: {err}"))?;

    let env = Environment::new();
    Ok(eval(&program, env))
}

fn test_integer_object(obj: &Object, expected: i64) -> Result<(), String> {
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

#[test]
fn test_eval_boolean_expression() -> Result<(), String> {
    let test_cases = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        test_boolean_object(&evaluated, expected)?;
    }

    Ok(())
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

#[test]
fn test_bang_operator() -> Result<(), String> {
    let test_cases = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        test_boolean_object(&evaluated, expected)?;
    }
    Ok(())
}

#[test]
fn test_minus_operator() -> Result<(), String> {
    let test_cases: Vec<(&str, i64)> = vec![("5", 5), ("-5", -5), ("11", 11), ("-11", -11)];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        test_integer_object(&evaluated, expected)?;
    }
    Ok(())
}

#[test]
fn test_if_else_expressions() -> Result<(), String> {
    let test_cases = vec![
        ("if(true) {10}", "10"),
        ("if(false) {10}", "null"),
        ("if(1) {10}", "10"),
        ("if(1 < 2) {10}", "10"),
        ("if(1 > 2) {10}", "null"),
        ("if(1 > 2) {10} else {20}", "20"),
        ("if(1 < 2) {10} else {20}", "10"),
    ];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        expected.parse::<i64>().map_or_else(
            |_e| test_null_object(&evaluated),
            |v| test_integer_object(&evaluated, v),
        )?;
    }

    Ok(())
}

fn test_null_object(evaluated: &Object) -> Result<(), String> {
    match evaluated {
        Object::NULL => Ok(()),
        _ => Err(String::from("evalutated is not a Null Object")),
    }
}

#[test]
fn test_return_statement() -> Result<(), String> {
    let test_cases = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2*5; 9;", 10),
        ("if(10 > 1) { if  (10>1) {return 10;} return 1;}", 10),
    ];
    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;
        test_integer_object(&evaluated, expected)?;
    }
    Ok(())
}

#[test]
fn test_error_handling() -> Result<(), String> {
    let test_cases = vec![
        ("5+true;", "type mismatch:INTEGER + BOOLEAN"),
        ("5+true;5", "type mismatch:INTEGER + BOOLEAN"),
        ("-true;", "unknown operator:-BOOLEAN"),
        ("true + false;", "unknown operator:BOOLEAN + BOOLEAN"),
        ("5;true+false;5", "unknown operator:BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) {true + false;}",
            "unknown operator:BOOLEAN + BOOLEAN",
        ),
        (
            "if (10 > 1) { if(10 > 1) { return true + false;} return 1;};",
            "unknown operator:BOOLEAN + BOOLEAN",
        ),
        (
            "\"Hello\" - \"World!\"",
            "unknown operator: STRING - STRING",
        ),
        (
            "{\"name\":\"Monkey\"}[fn(x) {x}]",
            "unusable as hash key: FUNCTION",
        ),
    ];

    for (input, expected) in test_cases {
        let evaluated = test_eval(input)?;

        if let Object::ERROR(msg) = evaluated {
            assert_eq!(expected, msg)
        } else {
            panic!("not an error object for input:{}, got={}", input, evaluated)
        }
    }
    Ok(())
}

#[test]
fn test_let_statements() -> Result<(), String> {
    let test_cases = vec![
        ("let a = 5;a", 5),
        ("let a = 5 * 5;a", 25),
        ("let a = 5; let b = a;b;", 5),
        ("let a = 5;let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in test_cases {
        let val = test_eval(input)?;
        test_integer_object(&val, expected)?;
    }

    Ok(())
}

#[test]
fn test_function_eval() -> Result<(), String> {
    let input = "fn(x) {x + 2;};";
    let val = test_eval(input)?;
    if let Object::FUNCTION(func) = val {
        assert_eq!(
            1,
            func.parameters.len(),
            "function has wrong parameters, expected 1; got={}",
            func.parameters.len()
        );

        assert_eq!(
            "x",
            func.parameters[0].to_string(),
            "parameter is not 'x', got = {}",
            func.parameters[0]
        );

        assert_eq!(
            "(x + 2)",
            func.body.to_string(),
            "body is not \"(x + 2)\". got = {}",
            func.body.to_string()
        )
    } else {
        panic!("Result object is not a function");
    }

    Ok(())
}

#[test]
fn test_functions_application() -> Result<(), String> {
    let test_cases: Vec<(&str, i64)> = vec![
        ("let identify = fn(x) {x;}; identify(5);", 5),
        ("let identify = fn(x) {return x;}; identify(5);", 5),
        ("let double = fn(x) {x * 2;}; double(5);", 10),
        ("let add = fn(x, y) {x + y;}; add(5,5);", 10),
        ("let add = fn(x,y) {x + y;}; add(5 + 5, add(5,5));", 20),
        ("fn(x) {x;}(5)", 5),
    ];

    for (input, expected) in test_cases {
        let val = test_eval(input)?;
        test_integer_object(&val, expected)?;
    }
    Ok(())
}

#[test]
fn test_closures() -> Result<(), String> {
    let input = "
        let newAdder = fn(x) {
            fn(y) {x + y};
        };
        let addTwo = newAdder(2);
        addTwo(2);
    ";
    let val = test_eval(input)?;
    test_integer_object(&val, 4.into())?;
    Ok(())
}

#[test]
fn test_string_literal() -> Result<(), String> {
    let input = "\"Hello World!\"";

    let val = test_eval(input)?;

    if let Object::STRING(str) = val {
        assert_eq!("Hello World!", str);
    } else {
        panic!("Result is not a STRING OBJECT")
    }

    Ok(())
}

#[test]
fn test_string_concatination() -> Result<(), String> {
    let input = "\"Hello\" + \" \" + \"World!\"";
    let val = test_eval(input)?;

    if let Object::STRING(str) = val {
        assert_eq!("Hello World!", str);
    } else {
        panic!("Result is not a STRING OBJECT")
    }

    Ok(())
}

#[test]
fn test_builtin_functions() -> Result<(), String> {
    let test_cases = vec![
        ("len(\"\")", "0"),
        ("len(\"four\")", "4"),
        ("len(\"hello world\")", "11"),
        ("len(1)", "argument to `len` not supported, got=INTEGER"),
        (
            "len(\"one\", \"two\")",
            "wrong number of arguments. got=2, want 1",
        ),
        ("len([1,2,3])", "3"),
    ];

    for (input, expected) in test_cases {
        let val = test_eval(input)?;
        let expect_clone = expected;
        expected.parse::<i64>().map_or_else(
            |_e| {
                if let Object::ERROR(msg) = &val {
                    assert_eq!(
                        msg, expect_clone,
                        "wrong error message. expected={expect_clone}, got={msg}"
                    );
                    Ok(())
                } else {
                    return Err(format!("object is not Error, got={}", val.ob_type()));
                }
            },
            |v| test_integer_object(&val, v),
        )?;
    }
    Ok(())
}

#[test]
fn test_array_literals() -> Result<(), String> {
    let input = "[1, 2*2, 3 + 3]";
    let eval = test_eval(input)?;

    if let Object::ARRAY(result) = eval {
        assert_eq!(
            result.len(),
            3,
            "array has wrong number of elements, got {}",
            result.len()
        );

        test_integer_object(&result[0], 1)?;
        test_integer_object(&result[1], 4)?;
        test_integer_object(&result[2], 6)?;

        Ok(())
    } else {
        return Err(format!("Object is not array, got {}", eval.ob_type()));
    }
}

#[test]
fn test_array_index_expressions() -> Result<(), String> {
    let test_cases = vec![
        ("[1,2,3][0]", "1"),
        ("[1,2,3][1]", "2"),
        ("[1,2,3][2]", "3"),
        ("let i = 0; [1][i]", "1"),
        ("[1,2,3][1+1]", "3"),
        ("let myArray=[1,2,3]; myArray[2];", "3"),
        (
            "let myArray=[1,2,3]; myArray[0] + myArray[1] + myArray[2];",
            "6",
        ),
        ("let myArray=[1,2,3];let i=myArray[0]; myArray[i];", "2"),
        ("[1,2,3][3]", "null"),
        ("[1,2,3][-1]", "null"),
    ];

    for (input, expect) in test_cases {
        let eval = test_eval(input)?;
        expect.parse::<i64>().map_or_else(
            |_e| test_null_object(&eval),
            |v| test_integer_object(&eval, v),
        )?;
    }

    Ok(())
}

#[test]
fn test_hash_literals() -> Result<(), String> {
    let input = "let two = \"two\";
            {
                \"one\": 10 - 9,
                two: 1 + 1,
                \"thr\" + \"ee\": 6/2,
                4: 4,
                true: 5,
                false: 6,
            }
        ";
    let eval = test_eval(input)?;

    if let Object::HASH(val) = eval {
        let expected: HashMap<HashKey, i64> = HashMap::from([
            (HashKey::new(Object::STRING("one".to_string())).unwrap(), 1),
            (HashKey::new(Object::STRING("two".to_string())).unwrap(), 2),
            (
                HashKey::new(Object::STRING("three".to_string())).unwrap(),
                3,
            ),
            (HashKey::new(Object::INTEGER(4)).unwrap(), 4),
            (HashKey::new(Object::BOOLEAN(true)).unwrap(), 5),
            (HashKey::new(Object::BOOLEAN(false)).unwrap(), 6),
        ]);

        assert_eq!(val.pairs.len(), expected.len());
        for (expected_key, expected_val) in expected {
            match val.pairs.get(&expected_key) {
                Some(v) => test_integer_object(&v.value, expected_val)?,
                None => return Err(format!("no pair for given key {:?} in Pairs", expected_key)),
            }
        }
    } else {
        return Err(format!("eval didn't return hash. got {}", eval.ob_type()));
    }
    Ok(())
}

#[test]
fn test_hash_index_expression() -> Result<(), String> {
    let test_cases = vec![
        ("{\"foo\": 5}[\"foo\"]", "5"),
        ("{\"foo\": 5}[\"bar\"]", "null"),
        ("let key = \"foo\";{\"foo\": 5}[key]", "5"),
        ("{}[\"foo\"]", "null"),
        ("{5:5}[5]", "5"),
        ("{true: 5}[true]", "5"),
        ("{false:5}[false]", "5"),
    ];

    for (input, expected) in test_cases {
        let eval = test_eval(input)?;
        expected.parse::<i64>().map_or_else(
            |_e| test_null_object(&eval),
            |v| test_integer_object(&eval, v),
        )?;
    }

    Ok(())
}

#[test]
fn test_quotes() -> Result<(), String> {
    let test_cases = vec![
        ("quote(5+8)", "(5 + 8)"),
        ("quote(foobar)", "foobar"),
        ("quote(foobar + barfoo)", "(foobar + barfoo)"),
    ];

    for (input, expected) in test_cases {
        let eval = test_eval(input)?;
        if let Object::QUOTE(v) = eval {
            match v.node.token_literal() {
                Some(_) => assert_eq!(v.node.to_string(), expected),
                None => return Err("quote.Node is null".to_string()),
            }
        } else {
            return Err(format!("expect Object::NODE. got {}", eval.ob_type()));
        }
    }

    Ok(())
}
