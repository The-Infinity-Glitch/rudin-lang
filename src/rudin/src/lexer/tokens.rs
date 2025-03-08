use crate::lexer;
use logos;

#[derive(logos::Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize))]
pub enum TokenKind {
    // Special tokens
    #[token("\n", lexer::word_callback)]
    NewLine,

    #[token(" ", lexer::word_callback)]
    Whitespace,

    #[token("\0", lexer::word_callback)]
    Eof,

    // Delimiters
    #[token("(", lexer::word_callback)]
    LeftParen,

    #[token(")", lexer::word_callback)]
    RightParen,

    #[token("[", lexer::word_callback)]
    LeftBracket,

    #[token("]", lexer::word_callback)]
    RightBracket,

    #[token("{", lexer::word_callback)]
    LeftBrace,

    #[token("}", lexer::word_callback)]
    RightBrace,

    // Punctuation
    #[token(".", lexer::word_callback)]
    Dot,

    #[token(",", lexer::word_callback)]
    Comma,

    #[token(";", lexer::word_callback)]
    Semicolon,

    #[token(":", lexer::word_callback)]
    Colon,

    // Keywords
    #[token("use", lexer::word_callback)]
    KwUse,

    #[token("func", lexer::word_callback)]
    KwFunc,

    #[token("var", lexer::word_callback)]
    KwVar,

    #[token("const", lexer::word_callback)]
    KwConst,

    #[token("struct", lexer::word_callback)]
    KwStruct,

    #[token("enum", lexer::word_callback)]
    KwEnum,

    #[token("pub", lexer::word_callback)]
    KwPub,

    #[token("priv", lexer::word_callback)]
    KwPriv,

    #[token("prot", lexer::word_callback)]
    KwProt,

    #[token("over", lexer::word_callback)]
    KwOver,

    #[token("class", lexer::word_callback)]
    KwClass,

    #[token("extends", lexer::word_callback)]
    KwExtends,

    #[token("impl", lexer::word_callback)]
    KwImpl,

    #[token("trait", lexer::word_callback)]
    KwTrait,

    #[token("new", lexer::word_callback)]
    KwNew,

    #[token("destroy", lexer::word_callback)]
    KwDestroy,

    #[token("if", lexer::word_callback)]
    KwIf,

    #[token("else", lexer::word_callback)]
    KwElse,

    #[token("elif", lexer::word_callback)]
    KwElif,

    #[token("while", lexer::word_callback)]
    KwWhile,

    #[token("loop", lexer::word_callback)]
    KwLoop,

    #[token("for", lexer::word_callback)]
    KwFor,

    #[token("return", lexer::word_callback)]
    KwReturn,

    #[token("break", lexer::word_callback)]
    KwBreak,

    #[token("continue", lexer::word_callback)]
    KwContinue,

    // Bult-in types (Keywords too)
    #[token("void", lexer::word_callback)]
    TyVoid,

    #[token("int", lexer::word_callback)]
    TyInt,

    #[token("float", lexer::word_callback)]
    TyFloat,

    #[token("double", lexer::word_callback)]
    TyDouble,

    #[token("bool", lexer::word_callback)]
    TyBool,

    #[token("char", lexer::word_callback)]
    TyChar,

    #[token("str", lexer::word_callback)]
    TyString,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", lexer::word_callback)]
    Identifier,

    #[regex("[0-9]+", lexer::word_callback)]
    Number,

    #[regex(r#"'([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*'"#, lexer::word_callback)]
    CharLiteral,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, lexer::word_callback)]
    StringLiteral,

    #[token("true", lexer::word_callback)]
    True,

    #[token("false", lexer::word_callback)]
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize, value: String) -> Self {
        Token {
            kind,
            line,
            column,
            value,
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
