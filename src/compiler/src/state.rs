pub enum CompilerFlags {}

pub struct State {
    pub flags: Vec<CompilerFlags>,
    pub output_file: String,
    pub input_files: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        State {
            flags: Vec::new(),
            output_file: String::new(),
            input_files: Vec::new(),
        }
    }
}
