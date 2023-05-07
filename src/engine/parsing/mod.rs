use super::ast::Program;

mod lexer;
mod parser;

pub trait Parser {
    fn parse(&self, source: String) -> Program;
}

pub struct BuiltinParser;

impl Parser for BuiltinParser {
    fn parse(&self, source: String) -> Program {
        let mut lexer = lexer::Lexer::new(source);
        let mut parser = parser::Parser::new(&mut lexer);
        parser.parse_program()
    }
}
