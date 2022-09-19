mod cli;
mod engine;
mod runtime;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        cli::interact::start();
    } else {
        cli::file::run(&args[1]);
    }
}
