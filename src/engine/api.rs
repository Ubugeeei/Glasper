use crate::engine::{
    eval::{evaluator::Evaluator, object::Object},
    handle_scope::HandleScope,
    parse::{ast::Program, parser::Parser},
    tokenize::lexer::Lexer,
};
use std::{collections::HashMap, io::Error};

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
    global_scope: Global,
}
impl Context {
    pub fn new(scope: HandleScope) -> Self {
        Self {
            scope,
            global_scope: Global::new(),
        }
    }

    pub fn global(&mut self) -> &mut Global {
        &mut self.global_scope
    }
}

pub struct Global {
    scope: HashMap<String, Object>,
}
impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}
impl Global {
    pub fn new() -> Self {
        Global {
            scope: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        self.scope.get(key)
    }

    pub fn set(&mut self, name: &str, ob: Object) {
        self.scope.insert(name.to_string(), ob);
    }
}

pub struct Script<'a> {
    ast: Program,
    context: &'a mut Context,
}
impl<'a> Script<'a> {
    pub fn compile(source: String, context: &'a mut Context) -> Self {
        let mut l = Lexer::new(source);
        let mut p = Parser::new(&mut l);
        Script {
            ast: p.parse_program(),
            context,
        }
    }
    pub fn run(&mut self) -> Result<Object, Error> {
        let mut ev = Evaluator::new(self.context);
        ev.eval(&self.ast)
    }
}
