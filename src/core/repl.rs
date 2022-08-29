use crate::core::{lexer::Lexer, parse::parser::Parser};
use std::io::{self, Write};

const PROMPT: &str = "> ";

pub fn start() {
    println!("Welcome to Glasper v0.1.0 ");
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if &input == "exit()\n" {
            println!("Bye!");
            break;
        }

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        dbg!(program);
    }
}
