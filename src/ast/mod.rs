use crate::token::Token;
use std::fmt::Display;
use std::rc::Rc;

pub enum NodeType {
    Program,
    Expression,
    Statement,
}

pub trait IsNode: Display {
    fn token(&self) -> Token;
    fn node_type() -> NodeType;
}

pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl IsNode for Program {
    fn token(&self) -> Token {
        if self.statements.len() > 0 {
            self.statements[0].token()
        } else {
            Token::none()
        }
    }

    fn node_type() -> NodeType {
        NodeType::Program
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.statements {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

pub enum Statement {
    LetStmt(LetStatement),
    RetStmt(ReturnStatement),
    ExpStmt(ExpressionStatement),
    BlcStmt(BlockStatement),
}

impl IsNode for Statement {
    fn token(&self) -> Token {
        match self {
            Statement::LetStmt(let_statement) => let_statement.stmt_token,
            Statement::RetStmt(return_statement) => return_statement.stmt_token,
            Statement::ExpStmt(expression_statement) => expression_statement.stmt_token,
            Statement::BlcStmt(block_statement) => block_statement.token,
        }
    }

    fn node_type() -> NodeType {
        NodeType::Statement
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token().to_string())
    }
}

pub struct LetStatement {
    pub stmt_token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = ", self.token_literal().unwrap(), self.name)?;
        write!(f, "{}", self.value)?;

        write!(f, ";")
    }
}

impl From<LetStatement> for Statement {
    fn from(v: LetStatement) -> Statement {
        Statement::LetStmt(v)
    }
}

pub struct ReturnStatement {
    pub stmt_token: Token,
    pub return_value: Box<Expression>,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.token_literal().unwrap())?;
        write!(f, "{}", self.return_value)?;
        write!(f, ";")
    }
}

impl From<ReturnStatement> for Statement {
    fn from(v: ReturnStatement) -> Statement {
        Statement::RetStmt(v)
    }
}

pub struct ExpressionStatement {
    pub stmt_token: Token,
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl From<ExpressionStatement> for Statement {
    fn from(v: ExpressionStatement) -> Statement {
        Statement::ExpStmt(v)
    }
}

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            write!(f, "{stmt}")?;
        }

        Ok(())
    }
}

impl From<BlockStatement> for Statement {
    fn from(v: BlockStatement) -> Statement {
        Statement::BlcStmt(v)
    }
}

#[derive(Clone)]
pub enum Expression {
    Identifier(Identifier),
    StringLit(StringLiteral),
    IntLit(IntegerLiteral),
    PreExp(Rc<PrefixExpression>),
    InExp(Rc<InfixExpression>),
    BoolLit(Boolean),
    IfExp(Rc<IfExpression>),
    FncLit(Rc<FunctionLiteral>),
    CallExp(Rc<CallExpression>),
    ArrayExp(Rc<ArrayExpression>),
    IndexExp(Rc<IndexExpression>),
    HashLit(Rc<HashLiteral>),
}

impl IsNode for Expression {
    fn token(&self) -> Token {
        match self {
            Expression::Identifier(identifier) => identifier.token,
            Expression::StringLit(string_literal) => string_literal.token,
            Expression::IntLit(integer_literal) => integer_literal.token,
            Expression::PreExp(prefix_expression) => prefix_expression.token,
            Expression::InExp(infix_expression) => infix_expression.token,
            Expression::BoolLit(boolean) => boolean.token,
            Expression::IfExp(if_expression) => if_expression.token,
            Expression::FncLit(function_literal) => function_literal.token,
            Expression::CallExp(call_expression) => call_expression.token,
            Expression::ArrayExp(array_expression) => array_expression.token,
            Expression::IndexExp(index_expression) => index_expression.token,
            Expression::HashLit(hash_literal) => hash_literal.token,
        }
    }

    fn node_type() -> NodeType {
        NodeType::Expression
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "{}", identifier.value),
            Expression::StringLit(string_literal) => write!(f, "{}", string_literal.token()),
            Expression::IntLit(integer_literal) => write!(f, "{}", integer_literal.token()),
            Expression::PreExp(prefix_expression) => write!(
                f,
                "({}{})",
                prefix_expression.operator, prefix_expression.right
            ),
            Expression::InExp(infix_expression) => write!(
                f,
                "({} {} {})",
                infix_expression.left, infix_expression.operator, infix_expression.right
            ),
            Expression::BoolLit(boolean) => write!(f, "{}", self.token()),
            Expression::IfExp(if_expression) => {
                write!(f, "if {} ", if_expression.condition)?;
                write!(f, "{}", if_expression.consequence)?;
                if let Some(alternative) = &if_expression.alternative {
                    write!(f, "{}", alternative)?;
                }
                Ok(())
            }
            Expression::FncLit(function_literal) => {
                write!(f, "{} (", function_literal.token())?;
                let para_lists = function_literal
                    .parameters
                    .iter()
                    .map(|para| para.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{para_lists} )")?;
                write!(f, "{}", function_literal.body)?;
                Ok(())
            }
            Expression::CallExp(call_expression) => {
                let args_list = call_expression
                    .arguments
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", call_expression.function, args_list)
            }
            Expression::ArrayExp(array_expression) => {
                let elem_string = array_expression
                    .elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{elem_string}]")
            }
            Expression::IndexExp(index_expression) => {
                write!(f, "({}[{}])", index_expression.left, index_expression.index)
            }
            Expression::HashLit(hash_literal) => {
                let pair_str = hash_literal
                    .pairs
                    .iter()
                    .map(|(k, v)| format!("{k}:{v}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{pair_str}}}")
            }
        }
    }
}

#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl From<Identifier> for Expression {
    fn from(v: Identifier) -> Expression {
        Expression::Identifier(v)
    }
}

#[derive(Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl From<IntegerLiteral> for Expression {
    fn from(v: IntegerLiteral) -> Expression {
        Expression::IntLit(v)
    }
}

#[derive(Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl From<StringLiteral> for Expression {
    fn from(v: StringLiteral) -> Expression {
        Expression::StringLit(v)
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Expression,
}

impl From<PrefixExpression> for Expression {
    fn from(v: PrefixExpression) -> Expression {
        Expression::PreExp(Rc::new(v))
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Expression,
    pub operator: String,
    pub right: Expression,
}

impl From<InfixExpression> for Expression {
    fn from(v: InfixExpression) -> Expression {
        Expression::InExp(Rc::new(v))
    }
}

#[derive(Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl From<Boolean> for Expression {
    fn from(v: Boolean) -> Expression {
        Expression::BoolLit(v)
    }
}

pub struct IfExpression {
    pub token: Token,
    pub condition: Expression,
    pub consequence: Rc<Statement>,
    pub alternative: Option<Rc<Statement>>,
}

impl From<IfExpression> for Expression {
    fn from(v: IfExpression) -> Expression {
        Expression::IfExp(Rc::new(v))
    }
}

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Expression>,
    pub body: Rc<Statement>,
}

impl From<FunctionLiteral> for Expression {
    fn from(v: FunctionLiteral) -> Expression {
        Expression::FncLit(Rc::new(v))
    }
}

pub struct CallExpression {
    pub token: Token,
    pub function: Rc<Expression>,
    pub arguments: Vec<Expression>,
}

impl From<CallExpression> for Expression {
    fn from(v: CallExpression) -> Expression {
        Expression::CallExp(Rc::new(v))
    }
}

pub struct ArrayExpression {
    pub token: Token,
    pub elements: Vec<Expression>,
}

impl From<ArrayExpression> for Expression {
    fn from(v: ArrayExpression) -> Expression {
        Expression::ArrayExp(Rc::new(v))
    }
}

pub struct IndexExpression {
    pub token: Token,
    pub left: Rc<Expression>,
    pub index: Expression,
}

impl From<IndexExpression> for Expression {
    fn from(v: IndexExpression) -> Expression {
        Expression::IndexExp(Rc::new(v))
    }
}

pub struct HashLiteral {
    pub token: Token,
    pub pairs: Vec<(Expression, Expression)>,
}

impl From<HashLiteral> for Expression {
    fn from(value: HashLiteral) -> Self {
        Self::HashLit(Rc::new(value))
    }
}

#[cfg(test)]
mod test;
