use crate::core::{lexer::Lexer, token::TokenType};
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
        let mut t = l.next_token();
        while t.token_type != TokenType::Eof {
            println!("{:?}", t);
            t = l.next_token();
        }
    }
}
