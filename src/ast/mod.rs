use crate::token::Token;
use std::fmt::Display;
use std::rc::Rc;

pub trait Node: Display {
    fn token_literal(&self) -> Option<&str>;
}

pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> Option<&str> {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            None
        }
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

pub struct LetStatement {
    pub stmt_token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.stmt_token.tok_literal)
    }
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

impl Node for ReturnStatement {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.stmt_token.tok_literal)
    }
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

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.stmt_token.tok_literal)
    }
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

impl Node for BlockStatement {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
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

pub enum Statement {
    LetStmt(LetStatement),
    RetStmt(ReturnStatement),
    ExpStmt(ExpressionStatement),
    BlcStmt(BlockStatement),
}

impl Statement {
    fn as_node(&self) -> &dyn Node {
        match self {
            Statement::LetStmt(let_statement) => let_statement,
            Statement::RetStmt(return_statement) => return_statement,
            Statement::ExpStmt(expression_statement) => expression_statement,
            Statement::BlcStmt(block_statement) => block_statement,
        }
    }
}

impl Node for Statement {
    fn token_literal(&self) -> Option<&str> {
        self.as_node().token_literal()
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_node())
    }
}

#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
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

impl Node for IntegerLiteral {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.tok_literal)
    }
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

impl Node for StringLiteral {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.tok_literal)
    }
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

impl Node for PrefixExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
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

impl Node for InfixExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
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

impl Node for Boolean {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.tok_literal)
    }
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

impl Node for IfExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} ", self.condition)?;
        write!(f, "{}", self.consequence)?;
        if let Some(alternative) = &self.alternative {
            write!(f, "{}", alternative)?;
        }
        Ok(())
    }
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

impl Node for FunctionLiteral {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (", self.token.tok_literal)?;
        let para_lists = self
            .parameters
            .iter()
            .map(|para| para.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} )", para_lists)?;
        write!(f, "{}", self.body)?;
        Ok(())
    }
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

impl Node for CallExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_list = self
            .arguments
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}({})", self.function, args_list)
    }
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

impl Node for ArrayExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for ArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elem_string = self
            .elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{elem_string}]")
    }
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

impl Node for IndexExpression {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
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

impl Node for HashLiteral {
    fn token_literal(&self) -> Option<&str> {
        Some(&self.token.tok_literal)
    }
}

impl Display for HashLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pair_str = self
            .pairs
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{{{}}}", pair_str)
    }
}

impl From<HashLiteral> for Expression {
    fn from(value: HashLiteral) -> Self {
        Self::HashLit(Rc::new(value))
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

impl Expression {
    fn as_node(&self) -> &dyn Node {
        match self {
            Expression::Identifier(identifier) => identifier,
            Expression::StringLit(string_literal) => string_literal,
            Expression::IntLit(integer_literal) => integer_literal,
            Expression::PreExp(prefix_expression) => prefix_expression.as_ref(),
            Expression::InExp(infix_expression) => infix_expression.as_ref(),
            Expression::BoolLit(boolean) => boolean,
            Expression::IfExp(if_expression) => if_expression.as_ref(),
            Expression::FncLit(function_literal) => function_literal.as_ref(),
            Expression::CallExp(call_expression) => call_expression.as_ref(),
            Expression::ArrayExp(array_expression) => array_expression.as_ref(),
            Expression::IndexExp(index_expression) => index_expression.as_ref(),
            Expression::HashLit(hash_literal) => hash_literal.as_ref(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> Option<&str> {
        self.as_node().token_literal()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_node())
    }
}

#[cfg(test)]
mod test;
