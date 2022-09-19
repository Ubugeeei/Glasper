use crate::engine::{
    eval::{environment::Environment, evaluator::Evaluator, object::Object},
    parse::{ast::Program, parser::Parser},
    tokenize::lexer::Lexer,
};
use std::io::Error;

pub struct Isolate {
    pub context: Context,
}
impl Isolate {
    pub fn new(context: Context) -> Self {
        Isolate { context }
    }
}

pub struct Context {
    pub scope: Environment,
}
impl Context {
    pub fn new(scope: Environment) -> Self {
        Self { scope }
    }
}

pub struct Script<'a> {
    ast: Program,
    scope: &'a mut Environment,
}
impl<'a> Script<'a> {
    pub fn compile(source: String, scope: &'a mut Environment) -> Self {
        let mut l = Lexer::new(source);
        let mut p = Parser::new(&mut l);
        Script {
            ast: p.parse_program(),
            scope,
        }
    }
    pub fn run(&mut self) -> Result<Object, Error> {
        let mut ev = Evaluator::new(self.scope);
        ev.eval(&self.ast)
    }
}
