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

    /// Parse a function call -> function_identifier(arguments)
    fn parse_function_call(&mut self) -> Option<parser::statements::Expression> {
        // The function identifier(name)
        let name: String = self.current().value.clone();
        self.advance();

        // '(' <- The start of the arguments
        self.advance();

        // A vector containing the current arguments of the function call
        let mut argument_vec: Vec<parser::statements::Expression> = Vec::new();

        // The current expression
        let mut expression: parser::statements::Expression;

        // While doesn't reaches the ')'
        while !self.peek_expect(&lexer::tokens::TokenKind::RightParen)
            || !self.peek_expect(&lexer::tokens::TokenKind::Eof)
        {
            // If reaches the ')' <- End of the arguments
            if self.peek_expect(&lexer::tokens::TokenKind::RightParen) {
                break;
            }

            // If reaches the EOF before the ')'
            if self.peek_expect(&lexer::tokens::TokenKind::Eof) {
                self.output
                    .push(handling::Message::expected_error("')'", self.current()));
                return None;
            }

            // If reaches ',' starts a new argument
            if self.peek_expect(&lexer::tokens::TokenKind::Comma) {
                self.advance();
                continue;
            }

            // Parse the expression for the current argument
            expression = match self.parse_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

            // If after the expression, is a ',' or ')', push the current argument and advance
            if self.peek_kind().eq(&lexer::tokens::TokenKind::Comma)
                || self.peek_kind().eq(&lexer::tokens::TokenKind::RightParen)
            {
                argument_vec.push(expression);
                self.advance();
            } else {
                self.advance();
                self.output.push(handling::Message::expected_error(
                    "',' or ')'",
                    self.current(),
                ));
                return None;
            }
        }

        // If the call doesn't have arguments, return a function without arguments ;)
        if argument_vec.len() == 0 {
            return Some(parser::statements::Expression::Call {
                name,
                arguments: None,
            });
        }

        // A full call with arguments
        return Some(parser::statements::Expression::Call {
            name,
            arguments: Some(Box::new(argument_vec)),
        });
    }

    /// Parse identifiers -> function calls, push identifier value...
    fn parse_identifier(&mut self) -> Option<parser::statements::Expression> {
        match self.current_kind() {
            lexer::tokens::TokenKind::Identifier => match self.peek_kind() {
                lexer::tokens::TokenKind::LeftParen => self.parse_function_call(),
                _ => Some(parser::statements::Expression::Identifier(
                    self.current().value.to_owned(),
                )),
            },
            _ => {
                self.output.push(handling::Message::expected_error(
                    "identifier",
                    self.current(),
                ));
                return None;
            }
        }
    }

    /// Parse the expression
    fn parse_primary_expression(&mut self) -> Option<parser::statements::Expression> {
        let token: lexer::tokens::Token = self.current().to_owned();

        match token.kind {
            lexer::tokens::TokenKind::Identifier => self.parse_identifier(),
            lexer::tokens::TokenKind::Number => Some(parser::statements::Expression::Literal {
                r#type: lexer::tokens::TokenKind::Number,
                value: self.current().value.to_owned(),
            }),
            lexer::tokens::TokenKind::StringLiteral => {
                Some(parser::statements::Expression::Literal {
                    r#type: lexer::tokens::TokenKind::StringLiteral,
                    value: self.current().value.to_owned(),
                })
            }
            lexer::tokens::TokenKind::True | lexer::tokens::TokenKind::False => {
                Some(parser::statements::Expression::Literal {
                    r#type: lexer::tokens::TokenKind::TyBool,
                    value: self.current().value.to_owned(),
                })
            }
            lexer::tokens::TokenKind::Eof => {
                self.output.push(handling::Message::expected_error(
                    "end of the expression",
                    &token,
                ));
                return None;
            }
            _ => {
                self.output
                    .push(handling::Message::expected_error("an expression", &token));
                return None;
            }
        }
    }

    /// Parse unary expressions
    fn parse_unary_expression(&mut self) -> Option<parser::statements::Expression> {
        match self.current_kind() {
            lexer::tokens::TokenKind::OpAdd => {
                self.advance();
                self.parse_unary_expression()
            }
            lexer::tokens::TokenKind::OpSub => {
                self.advance();
                Some(parser::statements::Expression::Unary {
                    operator: lexer::tokens::TokenKind::OpSub,
                    operand: Box::new(match self.parse_unary_expression() {
                        Some(expr) => expr,
                        None => {
                            return None;
                        }
                    }),
                })
            }
            lexer::tokens::TokenKind::OpNot => {
                self.advance();
                Some(parser::statements::Expression::Unary {
                    operator: lexer::tokens::TokenKind::OpNot,
                    operand: Box::new(match self.parse_unary_expression() {
                        Some(expr) => expr,
                        None => {
                            return None;
                        }
                    }),
                })
            }
            _ => self.parse_primary_expression(),
        }
    }

    /// '*', '/' or '%' <- Multiplication, division or rest expression
    fn parse_multiplicative_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression = match self.parse_unary_expression() {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpMul)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpDiv)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpMod)
        {
            self.advance();

            let operator: lexer::tokens::Token = self.current().clone();
            self.advance();

            let right: parser::statements::Expression = match self.parse_unary_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

            left = parser::statements::Expression::Binary {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// '+' or '-' <- Sum or subratction expression
    fn parse_plus_or_subtract_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression = match self.parse_multiplicative_expression()
        {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpAdd)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpSub)
        {
            self.advance();

            let operator = self.current().clone();
            self.advance();

            let right: parser::statements::Expression = match self.parse_multiplicative_expression()
            {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

            left = parser::statements::Expression::Binary {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// '<', '<=', '>' or '>=' <- Size expressions
    fn parse_greater_or_smaller_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression =
            match self.parse_plus_or_subtract_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpLt)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpLe)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpGt)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpGe)
        {
            self.advance();

            let operator = self.current().clone();
            self.advance();

            let right: parser::statements::Expression =
                match self.parse_plus_or_subtract_expression() {
                    Some(expr) => expr,
                    None => {
                        return None;
                    }
                };

            left = parser::statements::Expression::Logical {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// '==' or '!=' <- Comparision expressions
    fn parse_comparision_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression =
            match self.parse_greater_or_smaller_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpEq)
            || self.peek_kind().eq(&lexer::tokens::TokenKind::OpNeq)
        {
            self.advance();

            let operator = self.current().clone();
            self.advance();

            let right: parser::statements::Expression =
                match self.parse_greater_or_smaller_expression() {
                    Some(expr) => expr,
                    None => {
                        return None;
                    }
                };

            left = parser::statements::Expression::Logical {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// 'and' or '&&' <- Logical and expressions
    fn parse_and_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression = match self.parse_comparision_expression() {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpAnd) {
            self.advance();

            let operator = self.current().clone();
            self.advance();

            let right: parser::statements::Expression = match self.parse_comparision_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

            left = parser::statements::Expression::Logical {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// 'or' or '||' <- Logical or expressions
    fn parse_or_expression(&mut self) -> Option<parser::statements::Expression> {
        let mut left: parser::statements::Expression = match self.parse_and_expression() {
            Some(expr) => expr,
            None => {
                return None;
            }
        };

        while self.peek_kind().eq(&lexer::tokens::TokenKind::OpOr) {
            self.advance();

            let operator = self.current().clone();
            self.advance();

            let right: parser::statements::Expression = match self.parse_and_expression() {
                Some(expr) => expr,
                None => {
                    return None;
                }
            };

            left = parser::statements::Expression::Logical {
                operator: operator.kind,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        return Some(left);
    }

    /// Parsing expressions related function
    fn parse_expression(&mut self) -> Option<parser::statements::Expression> {
        self.parse_or_expression()
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

        match self.current().kind {
            lexer::tokens::TokenKind::OpAssign => {
                self.advance();
            }
            lexer::tokens::TokenKind::Semicolon => {
                return Some(parser::statements::Statement::VariableDeclaration {
                    start: var_token.position,
                    name,
                    r#type,
                    value: None,
                });
            }
            _ => {
                self.output.push(handling::Message::expected_error(
                    "assignment operator or end of statement",
                    self.current(),
                ));
                return None;
            }
        }

        let value = match self.parse_expression() {
            Some(expr) => expr,
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
            value: Some(value),
        });
    }
}
