use rudin;

fn main() {
    let input = include_str!("../../../tests/function_statements.rudin");

    let mut lexer: rudin::lexer::Lexer = rudin::lexer::Lexer::new(input.to_string());

    lexer.lex();

    if lexer.output.len() != 0 {
        dbg!(lexer.output);
        std::process::exit(1);
    }

    let mut parser: rudin::parser::Parser = rudin::parser::Parser::new(lexer.tokens);

    parser.parse();

    if parser.output.len() != 0 {
        dbg!(parser.output);
        std::process::exit(1);
    }

    dbg!(parser.ast);
}
