mod compiler;
mod state;

fn main() {
    // Receive arguments from the command line and remove the first argument (program name)
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    // Initialize the state (flags, output etc)
    let mut state: state::State = state::State::new();
    state.parse_args(args);

    // If we have errors
    // TODO: We have to handle this better
    if state.output.len() != 0 {
        dbg!(state.output);
        std::process::exit(1);
    }

    if state.show_help {
        println!("Rudin Compiler v{}", env!("CARGO_PKG_VERSION"));
        let help_message = include_str!("../../../resources/helps/compiler_help.txt");
        println!("{}", help_message);
        std::process::exit(0);
    }

    if state.show_version {
        println!("Rudin Compiler v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let mut compiler: compiler::Compiler = compiler::Compiler::new(state);
    compiler.compile();

    if compiler.output.len() != 0 {
        dbg!(compiler.output);
        std::process::exit(1);
    }
}
