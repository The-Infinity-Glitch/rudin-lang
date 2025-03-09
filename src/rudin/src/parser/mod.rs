pub mod statements;

use crate::*;

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<lexer::tokens::Token>>,
    current_token: lexer::tokens::Token,
    pub output: Vec<handling::Message>,
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
            output: Vec::new(),
            ast,
        }
    }

    pub fn parse(&mut self) {
        while self.current_token.kind != lexer::tokens::TokenKind::Eof {
            let statement: Option<parser::statements::Statement> = match self.current().kind {
                lexer::tokens::TokenKind::KwVar => self.parse_var_statement(),
                _ => {
                    self.output
                        .push(handling::Message::unexpected_error(self.current()));
                    None
                }
            };

            if let Some(statement) = statement {
                self.push_statement(statement);
            } else {
                break;
            }

            self.advance();
        }
    }

    fn push_statement(&mut self, statement: parser::statements::Statement) {
        match &mut self.ast {
            parser::statements::Statement::Program { body, .. } => {
                body.push(statement);
            }
            _ => {}
        }
    }

    fn current(&self) -> &lexer::tokens::Token {
        &self.current_token
    }

    fn current_kind(&self) -> &lexer::tokens::TokenKind {
        &self.current_token.kind
    }

    fn advance(&mut self) {
        match self.current().kind {
            lexer::tokens::TokenKind::Eof => {}
            _ => self.current_token = self.tokens.next().unwrap(),
        }
    }

    fn peek_kind(&mut self) -> &lexer::tokens::TokenKind {
        match self.tokens.peek() {
            Some(token) => &token.kind,
            None => &lexer::tokens::TokenKind::Eof,
        }
    }

    fn peek_expect(&mut self, expected: &lexer::tokens::TokenKind) -> bool {
        self.current_kind().eq(expected)
    }

    fn get_type(&mut self) -> Option<internals::types::Types> {
        match self.current_kind() {
            lexer::tokens::TokenKind::TyInt => Some(internals::types::Types::Int),
            lexer::tokens::TokenKind::TyFloat => Some(internals::types::Types::Float),
            lexer::tokens::TokenKind::TyBool => Some(internals::types::Types::Bool),
            lexer::tokens::TokenKind::TyChar => Some(internals::types::Types::Char),
            lexer::tokens::TokenKind::TyString => Some(internals::types::Types::String),
            lexer::tokens::TokenKind::TyVoid => Some(internals::types::Types::Void),
            lexer::tokens::TokenKind::Identifier => Some(internals::types::Types::Custom(
                self.current().value.clone(),
            )),
            _ => {
                self.output
                    .push(handling::Message::expected_error("a type", self.current()));
                return None;
            }
        }
    }

    /// Parse the parameters inside parenthesis -> (param: type, other_param: type)
    fn parse_params(&mut self) -> Option<Vec<parser::statements::FuncParam>> {
        // '('
        self.advance();

        // "name: type" <- without the space
        let mut param: parser::statements::FuncParam;
        let mut params: Vec<parser::statements::FuncParam> = Vec::new();

        // If doesn't have parameters
        if self.peek_expect(&lexer::tokens::TokenKind::LeftParen) {
            return Some(params);
        }

        // while doesn't reaches ')' or EOF
        while !self.peek_expect(&lexer::tokens::TokenKind::RightParen)
            || !self.peek_expect(&lexer::tokens::TokenKind::Eof)
        {
            // If the first piece of the param isn't a identifier (name)
            if !self.peek_expect(&lexer::tokens::TokenKind::Identifier) {
                self.output.push(handling::Message::expected_error(
                    "Identifier",
                    self.current(),
                ));
                return None;
            }

            let name: String = self.current().value.clone();
            self.advance();

            // After the name ':'
            if let Some(message) = handling::Message::expected_or_error(
                lexer::tokens::TokenKind::Colon,
                "\':\'",
                self.current(),
            ) {
                self.output.push(message);
                return None;
            }

            self.advance();

            // The type of the parameter
            let r#type: internals::types::Types = match self.get_type() {
                Some(r#type) => r#type,
                None => {
                    self.output
                        .push(handling::Message::expected_error("a type", self.current()));
                    return None;
                }
            };
            self.advance();

            param = parser::statements::FuncParam { name, r#type };

            // The end of the parameters or another parameter
            if self.peek_expect(&lexer::tokens::TokenKind::RightParen) {
                params.push(param.to_owned());
                break;
            } else if self.peek_expect(&lexer::tokens::TokenKind::Comma) {
                params.push(param.to_owned());
                self.advance();
            } else {
                self.output.push(handling::Message::expected_error(
                    "\',\' or \')\'",
                    self.current(),
                ));
                return None;
            }
        }

        if !self.peek_expect(&lexer::tokens::TokenKind::RightParen) {
            self.output
                .push(handling::Message::expected_error("\')\'", self.current()));
            return None;
        }

        return Some(params);
    }

    fn parse_var_statement(&mut self) -> Option<parser::statements::Statement> {
        // "var" -> token
        let var_token: lexer::tokens::Token = self.current().clone();
        self.advance();

        // The variable name
        let name: String = match self.current().kind {
            lexer::tokens::TokenKind::Identifier => self.current().value.clone(),
            _ => {
                self.output.push(handling::Message::expected_error(
                    "an identifier",
                    self.current(),
                ));
                return None;
            }
        };
        self.advance();

        if let Some(message) = handling::Message::expected_or_error(
            lexer::tokens::TokenKind::Colon,
            "a colon",
            self.current(),
        ) {
            self.output.push(message);
            return None;
        }
        self.advance();

        let r#type: internals::types::Types = match self.get_type() {
            Some(t) => t,
            None => {
                return None;
            }
        };
        self.advance();

        if let Some(message) = handling::Message::expected_or_error(
            lexer::tokens::TokenKind::Semicolon,
            "end of statement",
            self.current(),
        ) {
            self.output.push(message);
            return None;
        }

        return Some(parser::statements::Statement::VariableDeclaration {
            start: var_token.position,
            name,
            r#type,
            value: None,
        });
    }
}
