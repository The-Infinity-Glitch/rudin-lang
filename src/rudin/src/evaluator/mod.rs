use std::ops::Deref;

use crate::*;

pub struct Evaluator {
    parser_output: parser::statements::Statement,
    output: Vec<handling::Message>,
}

impl Evaluator {
    pub fn new(parser_output: parser::statements::Statement) -> Self {
        Self {
            parser_output,
            output: Vec::new(),
        }
    }

    pub fn evaluate(&mut self) {
        match self.parser_output.clone() {
            parser::statements::Statement::Program { body, .. } => {
                let unboxed_body = body.deref().clone();

                for statement in unboxed_body {
                    match statement.clone() {
                        parser::statements::Statement::VariableDeclaration { .. } => {
                            self.evaluate_variable_statement(statement)
                        }
                        parser::statements::Statement::ConstantDeclaration { .. } => {
                            self.evaluate_constant_statement(statement)
                        }
                        parser::statements::Statement::FunctionDeclaration { .. } => {
                            self.evaluate_function_statement(statement)
                        }
                        parser::statements::Statement::VariableAlteration { .. } => todo!(),
                        _ => {
                            self.output.push(handling::Message::new(
                                handling::MessageKind::Error,
                                "Invalid AST".to_string(),
                                None,
                            ));
                            break;
                        }
                    }
                }
            }
            _ => self.output.push(handling::Message::new(
                handling::MessageKind::Error,
                "Invalid AST".to_string(),
                None,
            )),
        }
    }

    fn evaluate_function_statement(&mut self, input: parser::statements::Statement) {
        println!("Evaluating function:");
        dbg!(input);
    }

    fn evaluate_variable_statement(&mut self, input: parser::statements::Statement) {
        println!("Evaluating variable:");
        dbg!(input);
    }

    fn evaluate_constant_statement(&mut self, input: parser::statements::Statement) {
        println!("Evaluating constant:");
        dbg!(input);
    }
}
