pub mod tokens;
use crate::*;

use logos::{self, Logos};

/// Update the line count and the char index.
pub fn newline_callback(lex: &mut logos::Lexer<tokens::TokenKind>) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
}

/// Compute the line and column position for the current word.
pub fn word_callback(lex: &mut logos::Lexer<tokens::TokenKind>) {
    let _line = lex.extras.0;
    let _column = lex.span().start - lex.extras.1;
}

pub struct Lexer {
    pub tokens: Vec<tokens::Token>,
    input: String,
    pub output: Vec<handling::Message>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            input,
            output: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        let mut lex: logos::Lexer<'_, tokens::TokenKind> =
            tokens::TokenKind::lexer(self.input.as_str());
        let mut token_position: (usize, usize) = (0, 0);

        while let Some(token_kind) = lex.next() {
            match token_kind {
                Ok(kind) => {
                    // If the token is a newline, update the line count and reset the column index, else increment the column index
                    match kind {
                        tokens::TokenKind::NewLine => {
                            token_position.0 += 1;
                            token_position.1 = 0;
                        }
                        _ => {
                            token_position.1 = lex.span().start - lex.extras.1;
                        }
                    }

                    let token: tokens::Token = tokens::Token::new(
                        kind,
                        internals::Position::new(token_position.0, token_position.1),
                        lex.slice().to_string(),
                    );

                    self.tokens.push(token);
                }
                Err(_) => {
                    let symbol: String = lex.slice().to_string();
                    let position: internals::Position =
                        internals::Position::new(token_position.0, token_position.1);

                    let error = handling::Message::new(
                        handling::MessageKind::CodeError,
                        format!("\"{}\" -> Undefined symbol", symbol),
                        Some(position),
                    );

                    self.output.push(error);
                }
            }

            let mut index: usize = 0;

            for token in self.tokens.clone() {
                if token.kind == tokens::TokenKind::Whitespace
                    || token.kind == tokens::TokenKind::NewLine
                {
                    self.tokens.remove(index);
                }

                index += 1;
            }
        }

        // Insert EOF token at the end of the tokens vector
        self.tokens.push(tokens::Token::new(
            tokens::TokenKind::Eof,
            internals::Position::new(token_position.0, token_position.1 + 1),
            String::from("EOF"),
        ));
    }
}
