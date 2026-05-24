use super::*;
use crate::ast::{IsNode, Statement};
use crate::lexer::Lexer;
use std::boxed::Box;
use std::collections::HashMap;

fn test_let_statement(stmt: &Statement, name: String) -> Result<(), String> {
    let stmt_literal = stmt.token().unwrap();
    assert_eq!(
        "let",
        stmt_literal.to_string(),
        "stmt_literal is not 'let', got ={}",
        stmt_literal
    );

    if let Statement::LetStmt(ref let_st) = *stmt {
        assert_eq!(
            name, let_st.name.value,
            "stmt.name.value not {}.got={}",
            name, let_st.name.value
        );
    } else {
        panic!("let stmt is not belong of kind letStmt")
    }

    Ok(())
}

#[test]
fn test_let_statments() -> Result<(), String> {
    let test_cases: Vec<(&str, &str, LiteralVal)> = vec![
        ("let x = 5;", "x", 5.into()),
        ("let y = true;", "y", true.into()),
        ("let foobar = y;", "foobar", "y".into()),
    ];

    for (input, expected_identifier, expected_value) in test_cases {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p
            .parse_program()
            .map_err(|err| format!("Parsing error: {err}"))?;

        assert_eq!(
            1,
            program.statements.len(),
            "program.Statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        let stmt = &program.statements[0];

        assert!(test_let_statement(&*stmt, expected_identifier.to_string()).is_ok());

        if let Statement::LetStmt(s) = &stmt.as_ref() {
            assert!(test_literal_expression(&s.value, expected_value).is_ok());
        }
    }

    Ok(())
}

#[test]
fn test_return_statements() -> Result<(), String> {
    let input = "
        return 5;
        return 10;
        return 993322;
    ";

    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = match p.parse_program() {
        Err(_e) => return Err(String::from("ParseProgram() error")),
        Ok(p) => p,
    };

    assert_eq!(
        3,
        program.statements.len(),
        "program.statement does not contain 3 statement. got={}",
        program.statements.len()
    );

    for stmt in program.statements {
        let stmt_literal = stmt.token().unwrap();
        assert_eq!(
            "return",
            stmt_literal.to_string(),
            "return_stmt token literal not 'return', got {}",
            stmt_literal
        )
    }

    Ok(())
}

#[test]
fn test_interget_literal_expression() -> Result<(), String> {
    let input = "5;";

    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = match p.parse_program() {
        Err(_e) => return Err(String::from("ParseProgram() error")),
        Ok(p) => p,
    };

    assert_eq!(
        1,
        program.statements.len(),
        "program has not enough statements.got={}",
        program.statements.len()
    );
    for st in program.statements {
        let ex_stmt = if let Statement::ExpStmt(es) = st.as_ref() {
            es
        } else {
            panic!("st is not a ExpressionStatement");
        };

        if let Expression::IntLit(il) = &ex_stmt.expression {
            assert_eq!(5, il.value, "literal.value not {}, got {}", 5, il.value);
            let tok_lit = il.token.clone();
            assert_eq!(
                "5",
                tok_lit.to_string(),
                "literal.TokenLiteral not {}, got {}",
                "5",
                tok_lit
            );
        } else {
            panic!("st.Expression is not a IntergerLiteral");
        }
    }

    Ok(())
}

#[test]
fn test_boolean_literal_expression() -> Result<(), String> {
    let input = "true;";

    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = match p.parse_program() {
        Err(_e) => return Err(String::from("ParseProgram() error")),
        Ok(p) => p,
    };

    assert_eq!(
        1,
        program.statements.len(),
        "program has not enough statements.got={}",
        program.statements.len()
    );
    for st in program.statements {
        let ex_stmt = if let Statement::ExpStmt(es) = st.as_ref() {
            es
        } else {
            panic!("st is not a ExpressionStatement");
        };

        if let Expression::BoolLit(bl) = &ex_stmt.expression {
            assert_eq!(
                true, bl.value,
                "literal.value not {}, got {}",
                true, bl.value
            );
            let tok_lit = bl.token.clone();
            assert_eq!(
                "true",
                tok_lit.to_string(),
                "literal.TokenLiteral not {}, got {}",
                "true",
                tok_lit
            );
        } else {
            panic!("st.Expression is not a BooleanLiteral");
        }
    }

    Ok(())
}

#[test]
fn test_parsing_prefix_expression() -> Result<(), String> {
    let prefix_tests: Vec<(&'static str, &'static str, LiteralVal)> = vec![
        ("!5", "!", 5.into()),
        ("-15", "-", 15.into()),
        ("!true;", "!", true.into()),
        ("!false;", "!", false.into()),
    ];

    for tcase in prefix_tests {
        let l = Lexer::new(tcase.0);
        let mut p = Parser::new(l);
        let program = p
            .parse_program()
            .map_err(|e| -> String { format!("parse error:{e}") })?;

        assert_eq!(
            1,
            program.statements.len(),
            "program.statements does not contain 1, got {}",
            program.statements.len()
        );

        let exp_stmt = if let Statement::ExpStmt(ex) = &program.statements[0].as_ref() {
            ex
        } else {
            panic!("program.statements[0] is not an Expression Statement")
        };

        if let Expression::PreExp(pe) = &exp_stmt.expression {
            assert_eq!(
                tcase.1, pe.operator,
                "exp.Opartor is not {}, got={}",
                tcase.1, pe.operator
            );
            assert!(test_literal_expression(&pe.right, tcase.2).is_ok());
        } else {
            panic!("exp_stmt is not a PrefixExpression")
        };
    }
    Ok(())
}

fn test_integer_literal(il: &Expression, value: i64) -> Result<(), String> {
    if let Expression::IntLit(literal) = il {
        assert_eq!(
            value, literal.value,
            "literal.value not {}, got{}",
            value, literal.value
        );

        let tok_lit = literal.token.clone();

        assert_eq!(
            value.to_string(),
            tok_lit.to_string(),
            "literal.token_literal() not {}, got {}",
            value.to_string(),
            tok_lit
        )
    } else {
        panic!("literal is not an Integer Literal")
    }
    Ok(())
}

fn test_boolean_literal(bl: &Expression, value: bool) -> Result<(), String> {
    if let Expression::BoolLit(literal) = bl {
        assert_eq!(
            value, literal.value,
            "literal.value not {}, got{}",
            value, literal.value
        );

        let tok_lit = literal.token.clone();

        assert_eq!(
            value.to_string(),
            tok_lit.to_string(),
            "literal.token_literal() not {}, got {}",
            value.to_string(),
            tok_lit
        )
    } else {
        panic!("literal is not an boolean Literal")
    }
    Ok(())
}

fn test_identifier(ie: &Expression, value: String) -> Result<(), String> {
    if let Expression::Identifier(ident) = ie {
        assert_eq!(
            value, ident.value,
            "ident.value not {value}. got={}",
            ident.value
        );
        let tok_lit = ident.token.clone();
        assert_eq!(
            value,
            tok_lit.to_string(),
            "ident.TokenLiteral not {value}, got {tok_lit}"
        );
    } else {
        panic!("ie is not Indentifier Expression")
    }
    Ok(())
}

#[derive(PartialEq, Debug)]
enum LiteralVal {
    Int(i64),
    Ident(String),
    Boolean(bool),
}

impl From<i64> for LiteralVal {
    fn from(value: i64) -> Self {
        LiteralVal::Int(value)
    }
}

impl From<&str> for LiteralVal {
    fn from(value: &str) -> Self {
        LiteralVal::Ident(value.to_string())
    }
}

impl From<bool> for LiteralVal {
    fn from(value: bool) -> Self {
        LiteralVal::Boolean(value)
    }
}

fn test_literal_expression(exp: &Expression, value: LiteralVal) -> Result<(), String> {
    match value {
        LiteralVal::Int(val) => test_integer_literal(exp, val),
        LiteralVal::Ident(id) => test_identifier(exp, id),
        LiteralVal::Boolean(bool) => test_boolean_literal(exp, bool),
    }
}

fn test_infix_expression(
    exp: &Expression,
    left: LiteralVal,
    operator: String,
    right: LiteralVal,
) -> Result<(), String> {
    if let Expression::InExp(infix_exp) = exp {
        assert!(test_literal_expression(&infix_exp.left, left).is_ok());
        assert_eq!(operator, infix_exp.operator);
        assert!(test_literal_expression(&infix_exp.right, right).is_ok());
        Ok(())
    } else {
        panic!("exp is not Infix Expression")
    }
}

#[test]
fn test_operator_precedence_parsing() -> Result<(), String> {
    let test_cases = vec![
        ("-a*b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4;-5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b*c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a,b,1,2*3,4 + 5, add(6,7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c*d/f+g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        ("a*[1,2,3,4][b*c]*d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        (
            "add(a*b[2],b[1],2*[1,2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    for (test_index, test) in test_cases.iter().enumerate() {
        let l = Lexer::new(test.0);
        let mut p = Parser::new(l);
        let program = p.parse_program().map_err(|err| -> String {
            format! {"parse error {err} at test {test_index}"}
        })?;

        let actual = program.to_string();

        assert_eq!(actual, test.1, "expected={}, got={actual}", test.1);
    }

    Ok(())
}

#[test]
fn test_parsing_infix_expression() -> Result<(), String> {
    struct Case {
        input: &'static str,
        left_val: LiteralVal,
        operator: &'static str,
        right_val: LiteralVal,
    }

    let test_cases: Vec<Case> = vec![
        Case {
            input: "5+5;",
            left_val: 5.into(),
            operator: "+",
            right_val: 5.into(),
        },
        Case {
            input: "5-5;",
            left_val: 5.into(),
            operator: "-",
            right_val: 5.into(),
        },
        Case {
            input: "5*5;",
            left_val: 5.into(),
            operator: "*",
            right_val: 5.into(),
        },
        Case {
            input: "5/5;",
            left_val: 5.into(),
            operator: "/",
            right_val: 5.into(),
        },
        Case {
            input: "5>5;",
            left_val: 5.into(),
            operator: ">",
            right_val: 5.into(),
        },
        Case {
            input: "5<5;",
            left_val: 5.into(),
            operator: "<",
            right_val: 5.into(),
        },
        Case {
            input: "5==5;",
            left_val: 5.into(),
            operator: "==",
            right_val: 5.into(),
        },
        Case {
            input: "5!=5;",
            left_val: 5.into(),
            operator: "!=",
            right_val: 5.into(),
        },
        Case {
            input: "true == true;",
            left_val: true.into(),
            operator: "==",
            right_val: true.into(),
        },
        Case {
            input: "true != false;",
            left_val: true.into(),
            operator: "!=",
            right_val: false.into(),
        },
        Case {
            input: "false == false;",
            left_val: false.into(),
            operator: "==",
            right_val: false.into(),
        },
    ];

    for tcase in test_cases {
        let l = Lexer::new(tcase.input);
        let mut p = Parser::new(l);
        let program = p
            .parse_program()
            .map_err(|err| -> String { format!("parse error{err}") })?;

        assert_eq!(
            1,
            program.statements.len(),
            "program statement does not contain 1 statement. got {}",
            program.statements.len()
        );

        let inf_stmt = if let Statement::ExpStmt(ex) = &program.statements[0].as_ref() {
            ex
        } else {
            panic!("program.statements[0] is not an Expression Statement")
        };

        assert!(test_infix_expression(
            &inf_stmt.expression,
            tcase.left_val,
            tcase.operator.to_string(),
            tcase.right_val
        )
        .is_ok())
    }

    Ok(())
}

#[test]
fn test_if_expression() -> Result<(), String> {
    let input = "if (x < y) {x}";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("parse error: {err}"))?;

    assert_eq!(
        1,
        program.statements.len(),
        "program.statements doesn't contain 1 statement. got={}",
        program.statements.len()
    );

    let if_stmt = if let Statement::ExpStmt(exp_stmt) = &program.statements[0].as_ref() {
        exp_stmt
    } else {
        panic!("program.statement[0] is not an Expression Statement");
    };

    let if_exp = if let Expression::IfExp(exp) = &if_stmt.expression {
        exp
    } else {
        panic!("if_stmt.expression is not an If Expression");
    };

    assert!(test_infix_expression(&if_exp.condition, "x".into(), "<".into(), "y".into()).is_ok());

    if let Statement::BlcStmt(bs) = if_exp.consequence.as_ref() {
        assert_eq!(
            1,
            bs.statements.len(),
            "consequences is not 1 statements. got={}",
            bs.statements.len()
        );

        let consequences_stmt = if let Statement::ExpStmt(stmt) = &bs.statements[0] {
            stmt
        } else {
            panic!("Consequence statements[0] is not an Expression Statement");
        };
        assert!(test_identifier(&consequences_stmt.expression, "x".into()).is_ok());
    } else {
        panic!("if_exp.consequence is not a Block Statement")
    }

    assert!(&if_exp.alternative.is_none());

    Ok(())
}

#[test]
fn test_if_else_expression() -> Result<(), String> {
    let input = "if (x < y) {x} else {y}";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("parse error: {err}"))?;

    assert_eq!(
        1,
        program.statements.len(),
        "program.statements doesn't contain 1 statement. got={}",
        program.statements.len()
    );

    let if_stmt = if let Statement::ExpStmt(exp_stmt) = &program.statements[0].as_ref() {
        exp_stmt
    } else {
        panic!("program.statement[0] is not an Expression Statement");
    };

    let if_exp = if let Expression::IfExp(exp) = &if_stmt.expression {
        exp
    } else {
        panic!("if_stmt.expression is not an If Expression");
    };

    assert!(test_infix_expression(&if_exp.condition, "x".into(), "<".into(), "y".into()).is_ok());

    if let Statement::BlcStmt(bs) = if_exp.consequence.as_ref() {
        assert_eq!(
            1,
            bs.statements.len(),
            "consequences is not 1 statements. got={}",
            bs.statements.len()
        );

        let consequences_stmt = if let Statement::ExpStmt(stmt) = &bs.statements[0] {
            stmt
        } else {
            panic!("Consequence statements[0] is not an Expression Statement");
        };
        assert!(test_identifier(&consequences_stmt.expression, "x".into()).is_ok());
    } else {
        panic!("the if_exp consequence is not a block statement")
    }

    let alternatives = if let Some(result) = &if_exp.alternative {
        result
    } else {
        panic!("No alternative block in if else expression")
    };

    if let Statement::BlcStmt(bs) = alternatives.as_ref() {
        assert_eq!(
            1,
            bs.statements.len(),
            "alternatives is not 1 statements. got={}",
            bs.statements.len()
        );

        let alternative_stmt = if let Statement::ExpStmt(stmt) = &bs.statements[0] {
            stmt
        } else {
            panic!("alternative statements[0] is not an Expression Statement");
        };
        assert!(test_identifier(&alternative_stmt.expression, "y".into()).is_ok());
    }
    Ok(())
}

#[test]
fn test_function_literal_parsing() -> Result<(), String> {
    let input = "fn(x,y) {x + y;}";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Program parsed error: {err}"))?;

    assert_eq!(
        1,
        program.statements.len(),
        "Program statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let exp_stmt = if let Statement::ExpStmt(exp_stmt) = &program.statements[0].as_ref() {
        exp_stmt
    } else {
        panic!("program.statements[0] is not a Expression Statement");
    };

    let fnc_lit = if let Expression::FncLit(exp) = &exp_stmt.expression {
        exp
    } else {
        panic!("exp_stmt expression is not Function Literal");
    };

    assert_eq!(
        2,
        fnc_lit.parameters.len(),
        "function literal parameter wrong, want 2, got ={}",
        fnc_lit.parameters.len()
    );

    assert!(test_identifier(&fnc_lit.parameters[0], "x".into()).is_ok());
    assert!(test_identifier(&fnc_lit.parameters[1], "y".into()).is_ok());

    if let Statement::BlcStmt(bs) = fnc_lit.body.as_ref() {
        assert_eq!(
            1,
            bs.statements.len(),
            "fnc_lit.body.statements has not 1 statements. got={}",
            bs.statements.len()
        );

        if let Statement::ExpStmt(body_stmt) = &bs.statements[0] {
            assert!(test_infix_expression(
                &body_stmt.expression,
                "x".into(),
                "+".into(),
                "y".into()
            )
            .is_ok());
        } else {
            panic!("fnc_lit.body.statements[0] is not an Expression Statement")
        }
    } else {
        panic!("fnc_lit.body.statements[0] is not a Block statement")
    }
    Ok(())
}

#[test]
fn test_function_parameter_parsing() -> Result<(), String> {
    let test_case = vec![
        ("fn() {}", vec![]),
        ("fn(x) {}", vec!["x"]),
        ("fn(x,y,z) {}", vec!["x", "y", "z"]),
    ];

    for (input, expected) in test_case {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p
            .parse_program()
            .map_err(|err| format!("Parsing Error: {err} with {input}"))?;

        let function = program
            .statements
            .get(0)
            .and_then(|stmt| match stmt.as_ref() {
                Statement::ExpStmt(s) => Some(s),
                _ => None,
            })
            .and_then(|exp_stmt| match &exp_stmt.expression {
                Expression::FncLit(s) => Some(s),
                _ => None,
            })
            .ok_or("Can't extract function literal from program.statements[0]")?;

        assert_eq!(
            expected.len(),
            function.parameters.len(),
            "parameters length is wrong. want {}, got{}",
            expected.len(),
            function.parameters.len()
        );

        for (index, &ident) in expected.iter().enumerate() {
            assert!(test_literal_expression(&function.parameters[index], ident.into()).is_ok());
        }
    }

    Ok(())
}

#[test]
fn test_call_expression_parsing() -> Result<(), String> {
    let input = "add(1,2*3,4+5);";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error: {err}"))?;

    assert_eq!(
        1,
        program.statements.len(),
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let exp = program
        .statements
        .get(0)
        .ok_or("can't find program.statement[0]")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(s) => Ok(s),
            _ => Err("program.statement[0] is not a Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::CallExp(s) => Ok(s),
            _ => Err("exp_stmt is not a Function call expression"),
        })?;

    test_identifier(&exp.function, "add".into())?;
    assert_eq!(
        3,
        exp.arguments.len(),
        "wrong arguments' length, got={}",
        exp.arguments.len()
    );

    test_literal_expression(&exp.arguments[0], 1.into())?;
    test_infix_expression(&exp.arguments[1], 2.into(), "*".into(), 3.into())?;
    test_infix_expression(&exp.arguments[2], 4.into(), "+".into(), 5.into())?;
    Ok(())
}

#[test]
fn test_string_literal_expression() -> Result<(), String> {
    let input = "\"hello world\";";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parssing error: {err}"))?;

    let str_lit = program
        .statements
        .get(0)
        .ok_or("Can't find program index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(s) => Ok(s),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::StringLit(str_lit) => Ok(str_lit),
            _ => Err("Expression is not a String Literal"),
        })?;

    assert_eq!(
        "hello world", str_lit.value,
        "str_lit.value not {}, got = {}",
        "hello world", str_lit.value
    );

    Ok(())
}

#[test]
fn test_parsing_array_literals() -> Result<(), String> {
    let input = "[1,2*2,3+3]";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error: {err}"))?;

    let array_exp = program
        .statements
        .get(0)
        .ok_or("Can't find program index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(s) => Ok(s),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::ArrayExp(array_exp) => Ok(array_exp),
            _ => Err("Expression is not an Array Literal"),
        })?;

    assert_eq!(
        array_exp.elements.len(),
        3,
        "array.elements.len() not 3. got={}",
        array_exp.elements.len()
    );

    test_integer_literal(&array_exp.elements[0], 1)?;
    test_infix_expression(
        &array_exp.elements[1],
        2.into(),
        String::from("*"),
        2.into(),
    )?;
    test_infix_expression(
        &array_exp.elements[2],
        3.into(),
        String::from("+"),
        3.into(),
    )?;

    Ok(())
}

#[test]
fn test_parsing_index_expression() -> Result<(), String> {
    let input = "myArray[1+1]";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let index_exp = program
        .statements
        .get(0)
        .ok_or("Can't find program statement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(es) => Ok(es),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::IndexExp(ide) => Ok(ide),
            _ => Err("Expression is not an Index Expression"),
        })?;

    test_identifier(&index_exp.left, "myArray".to_string())?;
    test_infix_expression(&index_exp.index, 1.into(), "+".to_string(), 1.into())?;

    Ok(())
}

#[test]
fn test_parsing_hash_literal_integer_keys() -> Result<(), String> {
    let input = "{1: 10, 2: 6, 3: 7}";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let harsh_lit = program
        .statements
        .get(0)
        .ok_or("Can't find program staement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(ex) => Ok(ex),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::HashLit(hl) => Ok(hl),
            _ => Err("Expression is not a Hash Literal"),
        })?;

    assert_eq!(
        3,
        harsh_lit.pairs.len(),
        "hash_lit.pairs has wrong length. got {}",
        harsh_lit.pairs.len()
    );

    let expected: HashMap<i64, i64> = HashMap::from([(1, 10), (2, 6), (3, 7)]);

    for (k, v) in &harsh_lit.pairs {
        if let Expression::IntLit(key) = k {
            match expected.get(&key.value) {
                Some(expected_val) => test_integer_literal(v, *expected_val)?,
                None => return Err(format!("Can't find expected_val for key {}", key.value)),
            }
        } else {
            return Err(format!("key is not a Integer Literal"));
        }
    }

    Ok(())
}

#[test]
fn test_parsing_hash_literal_boolean_keys() -> Result<(), String> {
    let input = "{true: 10, false: 6}";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let harsh_lit = program
        .statements
        .get(0)
        .ok_or("Can't find program staement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(ex) => Ok(ex),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::HashLit(hl) => Ok(hl),
            _ => Err("Expression is not a Hash Literal"),
        })?;

    assert_eq!(
        2,
        harsh_lit.pairs.len(),
        "hash_lit.pairs has wrong length. got {}",
        harsh_lit.pairs.len()
    );

    let expected: HashMap<bool, i64> = HashMap::from([(true, 10), (false, 6)]);

    for (k, v) in &harsh_lit.pairs {
        if let Expression::BoolLit(key) = k {
            match expected.get(&key.value) {
                Some(expected_val) => test_integer_literal(v, *expected_val)?,
                None => return Err(format!("Can't find expected_val for key {}", key.value)),
            }
        } else {
            return Err(format!("key is not a Integer Literal"));
        }
    }

    Ok(())
}

#[test]
fn test_parsing_hash_literal_string_keys() -> Result<(), String> {
    let input = "{\"one\": 1, \"two\": 2, \"three\": 3}";
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let harsh_lit = program
        .statements
        .get(0)
        .ok_or("Can't find program staement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(ex) => Ok(ex),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_stmt| match &exp_stmt.expression {
            Expression::HashLit(hl) => Ok(hl),
            _ => Err("Expression is not a Hash Literal"),
        })?;

    assert_eq!(
        3,
        harsh_lit.pairs.len(),
        "hash_lit.pairs has wrong length. got {}",
        harsh_lit.pairs.len()
    );

    let expected: HashMap<&str, i64> = HashMap::from([("one", 1), ("two", 2), ("three", 3)]);

    for (k, v) in &harsh_lit.pairs {
        if let Expression::StringLit(key) = k {
            match expected.get(key.value.as_str()) {
                Some(expected_val) => test_integer_literal(&v, *expected_val)?,
                None => return Err(format!("Can't find expected_val for key {}", key.value)),
            }
        } else {
            return Err(format!("key is not a String Literal"));
        }
    }

    Ok(())
}

#[test]
fn test_parsing_empty_hash_literal() -> Result<(), String> {
    let input = "{}";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let hash_lit = program
        .statements
        .get(0)
        .ok_or("Can't find statement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(exp_sta) => Ok(exp_sta),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_sta| match &exp_sta.expression {
            Expression::HashLit(hl) => Ok(hl),
            _ => Err("Expression is not an Hash Literal"),
        })?;

    assert_eq!(
        0,
        hash_lit.pairs.len(),
        "Hash lit length is not 0, got={}",
        hash_lit.pairs.len()
    );

    Ok(())
}

#[test]
fn test_parsing_hash_liteals_with_expression() -> Result<(), String> {
    let input = "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p
        .parse_program()
        .map_err(|err| format!("Parsing error:{err}"))?;

    let hash_lit = program
        .statements
        .get(0)
        .ok_or("Can't find statement at index 0")
        .and_then(|stmt| match stmt.as_ref() {
            Statement::ExpStmt(exp_sta) => Ok(exp_sta),
            _ => Err("Statement is not an Expression Statement"),
        })
        .and_then(|exp_sta| match &exp_sta.expression {
            Expression::HashLit(hl) => Ok(hl),
            _ => Err("Expression is not an Hash Literal"),
        })?;

    assert_eq!(
        3,
        hash_lit.pairs.len(),
        "hash_lit.pairs has wrong length. got {}",
        hash_lit.pairs.len()
    );

    type TestFn = Box<dyn Fn(&Expression) -> Result<(), String>>;

    let tests_fn_hash: HashMap<&str, TestFn> = HashMap::from([
        (
            "one",
            Box::new(|e: &Expression| test_infix_expression(e, 0.into(), "+".into(), 1.into()))
                as TestFn,
        ),
        (
            "two",
            Box::new(|e: &Expression| test_infix_expression(e, 10.into(), "-".into(), 8.into()))
                as TestFn,
        ),
        (
            "three",
            Box::new(|e: &Expression| test_infix_expression(e, 15.into(), "/".into(), 5.into()))
                as TestFn,
        ),
    ]);

    for (k, v) in &hash_lit.pairs {
        if let Expression::StringLit(key) = k {
            match tests_fn_hash.get(key.value.as_str()) {
                Some(test_fn) => test_fn(v)?,
                None => return Err(format!("No test function found for key {}", key.value)),
            }
        } else {
            return Err(format!("Key is not a StringLiteral. got {}", k));
        }
    }

    Ok(())
}
