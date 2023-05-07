use runtime::cli::{
    options::{get_execution_type, ExecutionType, HELP_MESSAGE},
    repl::start_repl,
    source::exec_source,
};
use std::env;

mod engine;
mod runtime;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();
    let execution_type = get_execution_type(&args);
    match execution_type {
        ExecutionType::Help => println!("{}", HELP_MESSAGE),
        ExecutionType::Version => println!("{}", VERSION),
        ExecutionType::VMInteract => start_repl(true),
        ExecutionType::HostInteract => start_repl(false),
        ExecutionType::VM { source_path } => exec_source(source_path, true),
        ExecutionType::Host { source_path } => exec_source(source_path, false),
    }
}
