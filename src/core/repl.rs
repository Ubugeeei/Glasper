use crate::core::{
    eval::evaluator::eval,
    lexer::Lexer,
    parse::{ast::Statement, parser::Parser},
};
use std::io::{self, BufRead, Write};

const PROMPT: &str = "> ";

pub fn start() {
    println!("Welcome to Glasper v0.1.0 ");

    let mut statements: Vec<Statement> = vec![];

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let input = stdin.lock().lines().map(|l| l.unwrap()).next().unwrap();

        if &input == "exit()\n" {
            println!("Bye!");
            break;
        }

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let mut program = p.parse_program();
        let res = eval(&program);
        println!("{}", res.unwrap());
        statements.append(&mut program.statements);
    }
}
