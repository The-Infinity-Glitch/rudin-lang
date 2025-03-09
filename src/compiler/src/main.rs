use rudin;

fn main() {
    let input = include_str!("../../../tests/var_statements.rudin");

    let mut lexer: rudin::lexer::Lexer = rudin::lexer::Lexer::new(input.to_string());

    lexer.lex();

    dbg!(lexer.tokens.clone());

    let mut parser: rudin::parser::Parser = rudin::parser::Parser::new(lexer.tokens);

    parser.parse();

    if parser.output.len() != 0 {
        dbg!(parser.output);
    }

    dbg!(parser.ast);
}
