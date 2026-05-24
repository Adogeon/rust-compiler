use crate::ast::{self, Expression, Statement};
use crate::lexer;
use crate::token::{self, TType};
use std::fmt::Display;
use std::rc::Rc;

#[derive(PartialOrd, PartialEq)]
enum Precedent {
    LOWEST = 0,
    EQUALS = 1,
    LESSGREATER = 2,
    SUM = 3,
    PRODUCT = 4,
    PREFIX = 5,
    CALL = 6,
    INDEX = 7,
}

fn tok_preceden_look_up(tok_type: TType) -> Precedent {
    match tok_type {
        TType::EQ => Precedent::EQUALS,
        TType::NOTEQ => Precedent::EQUALS,
        TType::LT => Precedent::LESSGREATER,
        TType::GT => Precedent::LESSGREATER,
        TType::PLUS => Precedent::SUM,
        TType::MINUS => Precedent::SUM,
        TType::SLASH => Precedent::PRODUCT,
        TType::ASTERISK => Precedent::PRODUCT,
        TType::ASSIGN => Precedent::PRODUCT,
        TType::LPAREN => Precedent::CALL,
        TType::LBRACKET => Precedent::INDEX,
        _ => Precedent::LOWEST,
    }
}

pub enum ParseError {
    UnexpectedToken(String),
    ParsingError,
    IntLitParseError(String),
    NoPrefixParseFnError(TType),
    NoInfixParseFnError(TType),
    MissingClosing(TType),
    MissingExpectedToken(TType),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(tok_lit) => {
                write!(f, "Parse Error - UnexpectedToken : {tok_lit}",)
            }
            Self::IntLitParseError(tok_lit) => write!(f, "can't parse {tok_lit} to int"),
            Self::NoPrefixParseFnError(tok_type) => write!(
                f,
                "Parse Error - No prefix parse function for {:?} found",
                tok_type
            ),
            Self::NoInfixParseFnError(tok_type) => write!(
                f,
                "Parse Error - No infix parse function for {:?} found",
                tok_type
            ),
            Self::MissingClosing(tok_type) => write!(f, "Parse Error - Missing {:?}", tok_type),
            Self::MissingExpectedToken(tok_type) => {
                write!(f, "Parse Error - Expecting {:?}", tok_type)
            }
            Self::ParsingError => write!(f, "Program has parsing Error"),
        }
    }
}

pub struct Parser<'a> {
    l: lexer::Lexer<'a>,
    cur_token: token::Token,
    peek_token: token::Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn next_tok(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn new(l: lexer::Lexer<'a>) -> Self {
        let mut p = Parser {
            l: l,
            cur_token: token::Token::new(TType::EOF, ""),
            peek_token: token::Token::new(TType::EOF, ""),
            errors: Vec::new(),
        };
        p.next_tok();
        p.next_tok();
        p
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut program = ast::Program { statements: vec![] };

        while self.cur_token.tok_type != TType::EOF {
            self.parse_statement()
                .map(|stmt| program.statements.push(Box::new(stmt)))
                .unwrap_or_else(|err| self.append_errors(err));

            self.next_tok()
        }

        if self.errors.len() > 0 {
            println!("parser has {} errors", self.errors.len());
            for err in &self.errors {
                println!("{err}");
            }
            return Err(ParseError::ParsingError);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.cur_token.tok_type {
            TType::LET => self.parse_let_stmt(),
            TType::RETURN => self.parse_return_stmt(),
            _ => self.parse_expression_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Statement, ParseError> {
        let token = self.cur_token.clone();

        self.expect_peek(TType::IDENT)
            .ok_or(ParseError::MissingExpectedToken(TType::IDENT))?;

        let stmt_name = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.tok_literal.clone(),
        };

        self.expect_peek(TType::ASSIGN)
            .ok_or(ParseError::MissingExpectedToken(TType::ASSIGN))?;

        self.next_tok();
        let value = self.parse_expression(Precedent::LOWEST)?;

        if self.peek_tok_is(TType::SEMICOLON).is_some() {
            self.next_tok();
        }

        Ok(ast::LetStatement {
            token,
            name: stmt_name,
            value: value,
        }
        .into())
    }

    fn parse_return_stmt(&mut self) -> Result<Statement, ParseError> {
        let token = self.cur_token.clone();
        self.next_tok();
        let stmt_value = self.parse_expression(Precedent::LOWEST)?;

        if self.peek_tok_is(TType::SEMICOLON).is_some() {
            self.next_tok();
        }

        Ok(ast::ReturnStatement {
            token,
            return_value: Box::new(stmt_value),
        }
        .into())
    }

    fn parse_expression_stmt(&mut self) -> Result<Statement, ParseError> {
        let token = self.cur_token.clone();
        let express = self.parse_expression(Precedent::LOWEST)?;
        if self.peek_tok_is(TType::SEMICOLON).is_some() {
            self.next_tok();
        }

        Ok(ast::ExpressionStatement {
            token,
            expression: express,
        }
        .into())
    }

    fn append_errors(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    fn cur_tok_is(&self, t: TType) -> bool {
        self.cur_token.tok_type == t
    }

    fn peek_tok_is(&self, t: TType) -> Option<()> {
        if self.peek_token.tok_type == t {
            Some(())
        } else {
            None
        }
    }

    fn expect_peek(&mut self, t: TType) -> Option<()> {
        self.peek_tok_is(t)?;
        self.next_tok();
        Some(())
    }

    fn cur_precedence(&self) -> Precedent {
        tok_preceden_look_up(self.cur_token.tok_type.clone())
    }

    fn peek_precedence(&self) -> Precedent {
        tok_preceden_look_up(self.peek_token.tok_type.clone())
    }

    fn prefix_parse_fn(&mut self, tok_type: TType) -> Result<Expression, ParseError> {
        match tok_type {
            TType::IDENT => self.parse_identifier(),
            TType::INT => self.parse_integer_literal(),
            TType::STRING => self.parse_string_literal(),
            TType::BANG | TType::MINUS => self.parse_prefix_expression(),
            TType::TRUE | TType::FALSE => self.parse_boolean(),
            TType::LPAREN => self.parse_grouped_expression(),
            TType::IF => self.parse_if_expression(),
            TType::FUNCTION => self.parse_function_literal(),
            TType::LBRACKET => self.parse_array_literal(),
            TType::LBRACE => self.parse_harsh_literal(),
            _ => Err(ParseError::NoPrefixParseFnError(tok_type)),
        }
    }

    fn check_infix_tok(&self, tok_type: &TType) -> bool {
        matches!(
            tok_type,
            TType::PLUS
                | TType::MINUS
                | TType::SLASH
                | TType::ASTERISK
                | TType::EQ
                | TType::NOTEQ
                | TType::LT
                | TType::GT
                | TType::LPAREN
                | TType::LBRACKET
        )
    }

    fn parse_expression(&mut self, precedence: Precedent) -> Result<Expression, ParseError> {
        let prefix = self.prefix_parse_fn(self.cur_token.tok_type.clone())?;

        let mut left_exp: Option<Box<Expression>> = Some(Box::new(prefix));
        while self.peek_tok_is(TType::SEMICOLON).is_none() && precedence < self.peek_precedence() {
            let peek_tok = self.peek_token.tok_type.clone();
            if self.check_infix_tok(&peek_tok) {
                self.next_tok();
                let box_left = left_exp.take().unwrap();
                let exp = match peek_tok {
                    TType::LPAREN => self.parse_call_expression(*box_left)?,
                    TType::LBRACKET => self.parse_index_expression(*box_left)?,
                    _ => self.parse_infix_expression(*box_left)?,
                };
                left_exp = Some(Box::new(exp));
            } else {
                break;
            }
        }
        left_exp.map(|exp| *exp).ok_or(ParseError::ParsingError)
    }

    fn parse_identifier(&self) -> Result<Expression, ParseError> {
        Ok(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.tok_literal.clone(),
        }
        .into())
    }

    fn parse_boolean(&self) -> Result<Expression, ParseError> {
        Ok(ast::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_tok_is(TType::TRUE),
        }
        .into())
    }

    fn parse_integer_literal(&self) -> Result<Expression, ParseError> {
        let int_token = self.cur_token.clone();
        let value: i64 = int_token
            .tok_literal
            .parse()
            .map_err(|_| ParseError::IntLitParseError(int_token.tok_literal.clone()))?;

        Ok(ast::IntegerLiteral {
            token: int_token,
            value,
        }
        .into())
    }

    fn parse_string_literal(&self) -> Result<Expression, ParseError> {
        Ok(ast::StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().tok_literal,
        }
        .into())
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let literal = self.cur_token.tok_literal.clone();

        self.next_tok();
        let right = self.parse_expression(Precedent::PREFIX)?;

        Ok(ast::PrefixExpression {
            token,
            operator: literal,
            right: right,
        }
        .into())
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let literal = self.cur_token.tok_literal.clone();
        let precedence = self.cur_precedence();

        self.next_tok();
        let right = self.parse_expression(precedence)?;

        Ok(ast::InfixExpression {
            token,
            operator: literal,
            left,
            right,
        }
        .into())
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_tok();
        let exp = self.parse_expression(Precedent::LOWEST)?;

        if self.expect_peek(TType::RPAREN).is_none() {
            return Err(ParseError::MissingClosing(TType::RPAREN));
        }

        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        let tok = self.cur_token.clone();
        if self.expect_peek(TType::LPAREN).is_none() {
            return Err(ParseError::MissingExpectedToken(TType::LPAREN));
        }

        self.next_tok();
        let condition = self.parse_expression(Precedent::LOWEST)?;

        if self.expect_peek(TType::RPAREN).is_none() {
            return Err(ParseError::MissingClosing(TType::RPAREN));
        }

        if self.expect_peek(TType::LBRACE).is_none() {
            return Err(ParseError::MissingExpectedToken(TType::LBRACE));
        }

        let consequence = self
            .parse_block_statement()
            .map_err(|_err| ParseError::ParsingError)?;

        let mut alternative = None;

        if self.peek_tok_is(TType::ELSE).is_some() {
            self.next_tok();
            if self.expect_peek(TType::LBRACE).is_none() {
                return Err(ParseError::MissingExpectedToken(TType::LBRACE));
            }
            alternative = Some(
                self.parse_block_statement()
                    .map_err(|_err| ParseError::ParsingError)?,
            );
        }

        Ok(ast::IfExpression {
            token: tok,
            condition: condition,
            consequence,
            alternative,
        }
        .into())
    }

    fn parse_block_statement(&mut self) -> Result<Rc<Statement>, ParseError> {
        let tok = self.cur_token.clone();
        let mut statements: Vec<Statement> = vec![];

        self.next_tok();
        while !self.cur_tok_is(TType::RBRACE) && !self.cur_tok_is(TType::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_tok()
        }

        Ok(Rc::new(
            ast::BlockStatement {
                token: tok,
                statements,
            }
            .into(),
        ))
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();

        if self.expect_peek(TType::LPAREN).is_none() {
            return Err(ParseError::MissingExpectedToken(TType::LPAREN));
        }

        let parameters = self
            .parse_function_parameters()
            .map_err(|_e| ParseError::ParsingError)?;

        if self.expect_peek(TType::LBRACE).is_none() {
            return Err(ParseError::MissingExpectedToken(TType::LBRACE));
        }

        let body = self
            .parse_block_statement()
            .map_err(|_e| ParseError::ParsingError)?;

        Ok(ast::FunctionLiteral {
            token,
            parameters,
            body,
        }
        .into())
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut identifiers: Vec<Expression> = vec![];

        if self.peek_tok_is(TType::RPAREN).is_some() {
            self.next_tok();
            return Ok(identifiers);
        }

        self.next_tok();
        let mut ident = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.clone().tok_literal,
        };
        identifiers.push(ident.into());

        while self.peek_tok_is(TType::COMMA).is_some() {
            self.next_tok();
            self.next_tok();
            ident = ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.clone().tok_literal,
            };
            identifiers.push(ident.into());
        }

        if self.expect_peek(TType::RPAREN).is_none() {
            return Err(ParseError::MissingExpectedToken(TType::RPAREN));
        }

        Ok(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let arguments = self.parse_expression_list(TType::RPAREN)?;

        Ok(ast::CallExpression {
            token,
            function: function.clone().into(),
            arguments,
        }
        .into())
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParseError> {
        let elements = self.parse_expression_list(TType::RBRACKET)?;

        Ok(ast::ArrayExpression {
            token: self.cur_token.clone(),
            elements,
        }
        .into())
    }

    fn parse_expression_list(&mut self, end: TType) -> Result<Vec<Expression>, ParseError> {
        let mut list: Vec<Expression> = vec![];

        if self.peek_tok_is(end.clone()).is_some() {
            self.next_tok();
            return Ok(list);
        }

        self.next_tok();
        list.push(self.parse_expression(Precedent::LOWEST)?);

        while self.peek_tok_is(TType::COMMA).is_some() {
            self.next_tok();
            self.next_tok();

            list.push(self.parse_expression(Precedent::LOWEST)?);
        }

        if self.expect_peek(end.clone()).is_none() {
            return Err(ParseError::MissingClosing(end));
        }
        Ok(list)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let left = left.clone().into();

        self.next_tok();

        let index = self.parse_expression(Precedent::LOWEST)?;

        if self.expect_peek(TType::RBRACKET).is_none() {
            return Err(ParseError::MissingClosing(TType::RBRACKET));
        }

        Ok(ast::IndexExpression { token, left, index }.into())
    }

    fn parse_harsh_literal(&mut self) -> Result<Expression, ParseError> {
        let token = self.cur_token.clone();
        let mut pairs: Vec<(Expression, Expression)> = Vec::new();

        while self.peek_tok_is(TType::RBRACE).is_none() {
            self.next_tok();
            let key = self.parse_expression(Precedent::LOWEST)?;

            if self.expect_peek(TType::COLON).is_none() {
                return Err(ParseError::MissingClosing(TType::COLON));
            }

            self.next_tok();
            let value = self.parse_expression(Precedent::LOWEST)?;
            pairs.push((key, value));

            if self.peek_tok_is(TType::RBRACE).is_none() && self.expect_peek(TType::COMMA).is_none()
            {
                return Err(ParseError::MissingExpectedToken(TType::COMMA));
            }
        }

        if self.expect_peek(TType::RBRACE).is_none() {
            return Err(ParseError::MissingClosing(TType::RBRACE));
        }

        Ok(ast::HashLiteral { token, pairs }.into())
    }
}

#[cfg(test)]
mod test;
