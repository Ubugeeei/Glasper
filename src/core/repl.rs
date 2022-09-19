use crate::core::{
    eval::{environment::Environment, evaluator::Evaluator},
    lexer::Lexer,
    parse::parser::Parser,
};
use std::io::{self, BufRead, Write};

const PROMPT: &str = "> ";

pub fn start() {
    println!("Welcome to Glasper v0.1.0 ");
    println!("exit using ctrl+c or ctrl+d or exit()");

    let mut e = Environment::new();
    let mut ev = Evaluator::new(&mut e);

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

        if &input == "exit()" {
            println!("Bye!");
            break;
        }

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        let res = ev.eval(&program);
        match res {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
    }
}
