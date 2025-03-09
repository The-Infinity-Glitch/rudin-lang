use crate::*;

#[derive(Debug, Clone)]
pub enum MessageKind {
    Error,
    CodeError,
    CodeWarning,
    Warning,
    CodeInfo,
    Info,
}

#[derive(Debug, Clone)]
pub struct Message {
    kind: MessageKind,
    message: String,
    position: Option<internals::Position>,
}

impl Message {
    pub fn new(kind: MessageKind, message: String, position: Option<internals::Position>) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }

    pub fn expected_error(expected: &str, found: &lexer::tokens::Token) -> Self {
        Self {
            kind: MessageKind::CodeError,
            message: format!("Expected {} but found {}", expected, found.value),
            position: Some(found.position.clone()),
        }
    }

    pub fn expected_or_error(
        expected_kind: lexer::tokens::TokenKind,
        expected: &str,
        found: &lexer::tokens::Token,
    ) -> Option<Self> {
        if expected_kind == found.kind {
            None
        } else {
            Some(Self {
                kind: MessageKind::CodeError,
                message: format!("Expected {} but found {}", expected, found.value),
                position: Some(found.position.clone()),
            })
        }
    }

    pub fn unexpected_error(found: &lexer::tokens::Token) -> Self {
        Self {
            kind: MessageKind::CodeError,
            message: format!("Unexpected token: {}", found.value),
            position: Some(found.position.clone()),
        }
    }
}
