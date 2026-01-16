use super::*;
use crate::token::{TType, Token};

#[test]
fn test_write_string() {
    let p = Program {
        statements: vec![Box::new(Statement::LetStmt(LetStatement {
            stmt_token: Token {
                tok_type: TType::LET,
                tok_literal: String::from("let"),
            },
            name: Identifier {
                token: Token {
                    tok_type: TType::IDENT,
                    tok_literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Expression::Identifier(Identifier {
                token: Token {
                    tok_type: TType::IDENT,
                    tok_literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            }),
        }))],
    };

    assert_eq!(
        p.to_string(),
        "let myVar = anotherVar;",
        "program.write_string() wrong, got={}",
        p.to_string()
    )
}

#[test]
fn test_modify() -> Result<(), String> {
    let one = || -> Expression {
        IntegerLiteral {
            value: 1,
            token: Token {
                tok_type: TType::INT,
                tok_literal: "1".to_string(),
            },
        }
        .into()
    };

    let two = || -> Expression {
        IntegerLiteral {
            value: 2,
            token: Token {
                tok_type: TType::INT,
                tok_literal: "2".to_string(),
            },
        }
        .into()
    };

    let turn_one_into_two = |mut node: Expression| -> Expression {
        if let Expression::IntLit(ref mut int_lit) = node {
            if int_lit.value == 1 {
                int_lit.value = 2;
            }
        }
        node
    };

    let test_cases: Vec<(&dyn Node, &dyn Node)> = vec![
        (&one(), &two()),
        (
            &Program {
                statements: vec![Box::new(Statement::ExpStmt(ExpressionStatement {
                    stmt_token: Token {
                        tok_type: TType::INT,
                        tok_literal: "1".to_string(),
                    },
                    expression: one(),
                }))],
            },
            &Program {
                statements: vec![Box::new(Statement::ExpStmt(ExpressionStatement {
                    stmt_token: Token {
                        tok_type: TType::INT,
                        tok_literal: "2".to_string(),
                    },
                    expression: two(),
                }))],
            },
        ),
    ];

    Ok(())
}
