use crate::*;
use logos;

#[derive(logos::Logos, Debug, Clone, PartialEq)]
#[logos(extras = (usize, usize))]
pub enum TokenKind {
    // Special tokens
    #[token("\n", lexer::newline_callback)]
    NewLine,

    #[token(" ", lexer::word_callback)]
    Whitespace,

    #[token("-#", lexer::word_callback)]
    OpenComment,

    #[token("#-", lexer::word_callback)]
    CloseComment,

    #[token("::", lexer::word_callback)]
    ColonColon,

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

    // Declaration keywords
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

    #[token("class", lexer::word_callback)]
    KwClass,

    #[token("trait", lexer::word_callback)]
    KwTrait,

    // Visibility modifiers keywords
    #[token("pub", lexer::word_callback)]
    KwPub,

    #[token("priv", lexer::word_callback)]
    KwPriv,

    #[token("prot", lexer::word_callback)]
    KwProt,

    // Logical keywords
    #[token("if", lexer::word_callback)]
    KwIf,

    #[token("else", lexer::word_callback)]
    KwElse,

    #[token("elif", lexer::word_callback)]
    KwElif,

    // Loop keywords
    #[token("while", lexer::word_callback)]
    KwWhile,

    #[token("loop", lexer::word_callback)]
    KwLoop,

    #[token("for", lexer::word_callback)]
    KwFor,

    // Control flow keywords
    #[token("return", lexer::word_callback)]
    KwReturn,

    #[token("break", lexer::word_callback)]
    KwBreak,

    #[token("continue", lexer::word_callback)]
    KwContinue,

    // Special keywords
    #[token("use", lexer::word_callback)]
    KwUse,

    #[token("over", lexer::word_callback)]
    KwOver,

    #[token("extends", lexer::word_callback)]
    KwExtends,

    #[token("impl", lexer::word_callback)]
    KwImpl,

    #[token("new", lexer::word_callback)]
    KwNew,

    #[token("destroy", lexer::word_callback)]
    KwDestroy,

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

    // Binary operators
    #[token("+", lexer::word_callback)]
    OpAdd,

    #[token("-", lexer::word_callback)]
    OpSub,

    #[token("*", lexer::word_callback)]
    OpMul,

    #[token("/", lexer::word_callback)]
    OpDiv,

    #[token("%", lexer::word_callback)]
    OpMod,

    // Logical operators
    #[token("==", lexer::word_callback)]
    OpEq,

    #[token("!=", lexer::word_callback)]
    OpNeq,

    #[token("&&", lexer::word_callback)]
    #[token("and", lexer::word_callback)]
    OpAnd,

    #[token("||", lexer::word_callback)]
    #[token("or", lexer::word_callback)]
    OpOr,

    #[token("!", lexer::word_callback)]
    #[token("not", lexer::word_callback)]
    OpNot,

    #[token("<", lexer::word_callback)]
    OpLt,

    #[token(">", lexer::word_callback)]
    OpGt,

    #[token("<=", lexer::word_callback)]
    OpLe,

    #[token(">=", lexer::word_callback)]
    OpGe,

    // Assignment operators
    #[token("=", lexer::word_callback)]
    OpAssign,

    #[token("+=", lexer::word_callback)]
    OpAddAssign,

    #[token("-=", lexer::word_callback)]
    OpSubAssign,

    #[token("*=", lexer::word_callback)]
    OpMulAssign,

    #[token("/=", lexer::word_callback)]
    OpDivAssign,

    #[token("%=", lexer::word_callback)]
    OpModAssign,

    // Special operators
    #[token("++", lexer::word_callback)]
    OpInc,

    #[token("--", lexer::word_callback)]
    OpDec,

    #[token("->", lexer::word_callback)]
    OpArrow,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", lexer::word_callback)]
    Identifier,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", lexer::word_callback)]
    Number,

    #[regex(r#"'([^'\\]|\\['\\bnfrt]|u[a-fA-F0-9]{4})*'"#, lexer::word_callback)]
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
    pub position: internals::Position,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, position: internals::Position, value: String) -> Self {
        Self {
            kind,
            position,
            value,
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.position.line, self.position.column)
    }
}
