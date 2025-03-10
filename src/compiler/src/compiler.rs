use crate::*;

pub struct Compiler {
    state: state::State,
    pub output: Vec<rudin::handling::Message>,
}

impl Compiler {
    pub fn new(state: state::State) -> Self {
        Compiler {
            state,
            output: Vec::new(),
        }
    }

    pub fn compile(&mut self) {
        let mut file_percentage: f64;
        let mut file_index: usize = 0;

        for file in self.state.input_files.iter() {
            file_index += 1;

            let content = match std::fs::read_to_string(file) {
                Ok(content) => content,
                Err(err) => {
                    self.output.push(rudin::handling::Message::new(
                        rudin::handling::MessageKind::Error,
                        format!("Error reading file '{}': {}", file, err.to_string()),
                        None,
                    ));
                    continue;
                }
            };

            file_percentage = file_index as f64 / self.state.input_files.len() as f64 * 100.0;

            println!("[{}%] Compiling file: {}", file_percentage, file);

            let mut lexer: rudin::lexer::Lexer = rudin::lexer::Lexer::new(content);
            lexer.lex();

            if lexer.output.len() != 0 {
                self.collect_messages(lexer.output);
                break;
            }

            let mut parser: rudin::parser::Parser = rudin::parser::Parser::new(lexer.tokens);
            parser.parse();

            if parser.output.len() != 0 {
                self.collect_messages(parser.output);
                break;
            }

            dbg!(parser.ast);
        }
    }

    fn collect_messages(&mut self, input: Vec<rudin::handling::Message>) {
        self.output.extend(input);
    }
}
