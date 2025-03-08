pub mod statements;

use crate::*;

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<lexer::tokens::Token>>,
    current_token: lexer::tokens::Token,
    pub ast: statements::Statement,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::tokens::Token>) -> Self {
        let mut tokens = tokens.into_iter().peekable();
        let current_token = tokens.next().unwrap();
        let ast = statements::Statement::Program {
            start: internals::Position::new(0, 0),
            body: Box::new(Vec::new()),
        };

        Self {
            tokens,
            current_token,
            ast,
        }
    }

    pub fn parse(&mut self) {
        while self.current_token.kind != lexer::tokens::TokenKind::Eof {}
    }
}
