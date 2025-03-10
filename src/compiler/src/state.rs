use rudin;

#[derive(Debug)]
pub enum CompilerFlags {
    CompilerDebug,
}

#[derive(Debug)]
pub struct State {
    pub flags: Vec<CompilerFlags>,
    pub output_file: String,
    pub input_files: Vec<String>,
    pub show_help: bool,
    pub show_version: bool,
    pub output: Vec<rudin::handling::Message>,
}

impl State {
    pub fn new() -> Self {
        State {
            flags: Vec::new(),
            output_file: String::new(),
            input_files: Vec::new(),
            show_help: false,
            show_version: false,
            output: Vec::new(),
        }
    }

    pub fn parse_args(&mut self, args: Vec<String>) {
        if args.is_empty() {
            self.show_help = true;
            return;
        }

        let mut args_inter: std::iter::Peekable<std::vec::IntoIter<String>> =
            args.into_iter().peekable();

        while let Some(arg) = args_inter.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    self.show_help = true;
                }
                "-v" | "--version" => {
                    self.show_version = true;
                }
                "-o" | "--output" => {
                    if let Some(file) = args_inter.next() {
                        self.output_file = file;
                    } else {
                        self.output.push(rudin::handling::Message::new(
                            rudin::handling::MessageKind::Error,
                            "Missing output file".to_string(),
                            None,
                        ));
                        break;
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        if let Some(error) = self.set_flag(arg.to_string()) {
                            self.output.push(error);
                            break;
                        }
                    } else {
                        self.input_files.push(arg);
                    }
                }
            }
        }
    }

    fn set_flag(&mut self, flag: String) -> Option<rudin::handling::Message> {
        match flag.as_str() {
            "-cdbg" => {
                self.flags.push(CompilerFlags::CompilerDebug);
                return None;
            }
            _ => {
                return Some(rudin::handling::Message::new(
                    rudin::handling::MessageKind::Error,
                    format!("Unknown flag: {}", flag),
                    None,
                ));
            }
        }
    }
}
