use crate::*;

pub struct Evaluator {
    parser_output: parser::statements::Statement,
}

impl Evaluator {
    pub fn new(parser_output: parser::statements::Statement) -> Self {
        Self { parser_output }
    }

    pub fn evaluate(&mut self) {}
}
