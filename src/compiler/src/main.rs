mod state;

use rudin;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let state: state::State = match parse_args(args) {
        Ok(state) => state,
        Err(err) => {
            dbg!(err);
            std::process::exit(1);
        }
    };
}

fn parse_args(args: Vec<String>) -> Result<state::State, Vec<rudin::handling::Message>> {
    let mut args_inter: std::iter::Peekable<std::vec::IntoIter<String>> =
        args.into_iter().peekable();
    let mut errors: Vec<rudin::handling::Message> = Vec::new();

    while let Some(arg) = args_inter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: rudin [options] [file]");
                println!("Options:");
                println!("  -h, --help      Display this help message");
                println!("  -v, --version   Display the version");
                std::process::exit(0);
            }
            "-v" | "--version" => {
                println!("rudin {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => {
                errors.push(rudin::handling::Message::new(
                    rudin::handling::MessageKind::Error,
                    format!("Unknown option: {}", arg),
                    None,
                ));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(state::State::new())
}
