use crate::runtime::js::JavaScriptRuntime;
use std::io::{self, BufRead, Write};

const PROMPT: &str = "> ";

pub fn start() {
    println!("Welcome to Glasper v0.1.0 ");
    println!("exit using ctrl+c or ctrl+d or exit()");
    let mut runtime = JavaScriptRuntime::new();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

        if &input == "exit()" {
            println!("Bye!");
            break;
        }

        match runtime.execute(input) {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
    }
}
