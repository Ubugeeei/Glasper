mod engine;
mod runtime;

use std::env;

const HELP: &str = r#"
Usage: gls [options] [ script.js ] [arguments]

Options:
    -                             script read from stdin (default if no file name is provided, interactive mode if a tty)
    -v, --version                 print version
    -h, --help                    print command line options (currently set)
    --vm                          run in vm mode (currently set)
"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();
    let vm = args.iter().any(|arg| arg == "--vm");

    if args.len() == 1 {
        runtime::cli::interact::start(vm);
    } else {
        match &*args[1] {
            "-h" | "--help" => {
                println!("{}", HELP);
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
            }
            "--vm" => {
                runtime::cli::interact::start(vm);
            }
            arg => {
                runtime::cli::file::run(arg, vm);
            }
        }
    }
}
