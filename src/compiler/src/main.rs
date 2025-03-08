use rudin;

fn main() {
    let input = include_str!("../../../tests/hello_world.rudin");

    let mut lexer: rudin::lexer::Lexer = rudin::lexer::Lexer::new(input.to_string());

    lexer.lex();

    dbg!(lexer.tokens.clone());

    let parser: rudin::parser::Parser = rudin::parser::Parser::new(lexer.tokens);
}
