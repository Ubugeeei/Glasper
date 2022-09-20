use crate::engine::{
    eval::{evaluator::Evaluator, object::Object},
    handle_scope::HandleScope,
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
    pub scope: HandleScope,
}
impl Context {
    pub fn new(scope: HandleScope) -> Self {
        Self { scope }
    }
}

pub struct Script<'a> {
    ast: Program,
    scope: &'a mut HandleScope,
}
impl<'a> Script<'a> {
    pub fn compile(source: String, scope: &'a mut HandleScope) -> Self {
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
