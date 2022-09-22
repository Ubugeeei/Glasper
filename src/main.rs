mod cli;
mod engine;
mod runtime;

use std::env;

const HELP: &str = r#"
Usage: gls [options] [ script.js ] [arguments]

Options:
    -                             script read from stdin (default if no file name is provided, interactive mode if a tty)
    -v, --version                 print version
    -h, --help                    print command line options (currently set)
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        cli::interact::start();
    } else {
        match &*args[1] {
            "-h" | "--help" => {
                println!("{}", HELP);
            }
            "-v" | "--version" => {}
            arg => {
                cli::file::run(arg);
            }
        }
    }
}
