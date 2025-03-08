pub mod tokens;
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
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            input,
        }
    }

    pub fn lex(&mut self) {
        let mut lex = tokens::TokenKind::lexer(self.input.as_str());
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

                    let token = tokens::Token::new(
                        kind,
                        token_position.0,
                        token_position.1,
                        lex.slice().to_string(),
                    );

                    self.tokens.push(token);
                }
                Err(_) => {}
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
    }
}
