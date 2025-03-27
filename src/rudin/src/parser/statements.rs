use crate::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Program {
        start: internals::Position,
        body: Box<Vec<Statement>>,
    },
    VariableDeclaration {
        start: internals::Position,
        name: String,
        r#type: internals::types::Types,
        value: Option<Expression>,
    },
    ConstantDeclaration {
        start: internals::Position,
        name: String,
        r#type: internals::types::Types,
        value: Expression,
    },
    FunctionDeclaration {
        start: internals::Position,
        name: String,
        r#type: internals::types::Types,
        params: Option<Vec<FuncParam>>,
        body: Option<Box<Vec<Statement>>>,
    },
    If {
        start: internals::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    ElseIf {
        start: internals::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Else {
        start: internals::Position,
        body: Option<Box<Vec<Statement>>>,
    },
    While {
        start: internals::Position,
        condition: Expression,
        body: Option<Box<Vec<Statement>>>,
    },
    For {
        start: internals::Position,
        variable: Option<Box<Statement>>,
        condition: Option<Expression>,
        variable_update: Option<Box<Statement>>,
        body: Option<Box<Vec<Statement>>>,
        alternate: Option<Box<Vec<Statement>>>,
    },
    Break {
        start: internals::Position,
    },
    Continue {
        start: internals::Position,
    },
    Return {
        start: internals::Position,
        expression: Option<Expression>,
    },
    VariableAlteration {
        name: String,
        operator: lexer::tokens::TokenKind,
        value: Expression,
    },
    FunctionCall(Expression),
}

#[derive(Debug, Clone)]
pub enum Loop {
    Yes,
    No,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub r#type: internals::types::Types,
}

#[derive(Debug, Clone)]
pub enum ArrayAcess {
    Acess {
        name: String,
        index: Box<Expression>,
    },
    NestedAcess {
        acess: Box<ArrayAcess>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Binary {
        operator: lexer::tokens::TokenKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Logical {
        operator: lexer::tokens::TokenKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: lexer::tokens::TokenKind,
        operand: Box<Expression>,
    },
    Literal {
        r#type: lexer::tokens::TokenKind,
        value: String,
    },
    ArrayLiteral {
        elements: Option<Box<Vec<Expression>>>,
    },
    ArrayAcess(ArrayAcess),
    Call {
        name: String,
        arguments: Option<Box<Vec<Expression>>>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Numeric,
    String,
    Boolean,
}
