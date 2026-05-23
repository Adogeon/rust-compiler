use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone)]
pub enum TType {
    ILLEGAL,
    EOF,
    // Identifiers + Literals
    IDENT,
    INT,
    STRING,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
    //DELIMITERS
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COLON,
    //KEYWORD
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn look_up_ident(ident: &str) -> TType {
    match ident {
        "fn" => TType::FUNCTION,
        "let" => TType::LET,
        "true" => TType::TRUE,
        "false" => TType::FALSE,
        "if" => TType::IF,
        "else" => TType::ELSE,
        "return" => TType::RETURN,
        _ => TType::IDENT,
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok_type: TType,
    pub tok_literal: String,
}

impl Token {
    pub fn new(ttype: TType, literal: &str) -> Token {
        Token {
            tok_type: ttype,
            tok_literal: literal.to_string(),
        }
    }

    pub fn none() -> Token {
        Token {
            tok_type: TType::EOF,
            tok_literal: String::from("NONE"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tok_literal)
    }
}
