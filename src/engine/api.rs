use crate::engine::{
    eval::{evaluator::Evaluator, object::RuntimeObject},
    handle_scope::HandleScope,
    parse::{ast::Program, parser::Parser},
    tokenize::lexer::Lexer,
};
use std::{cell::RefCell, collections::HashMap, io::Error, rc::Rc};

use super::eval::object::JSObject;

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
    scope: HashMap<String, RuntimeObject>,
}
impl Default for Global {
    fn default() -> Self {
        Self::new()
    }
}
impl Global {
    pub fn new() -> Self {
        let mut scope = HashMap::new();

        // install array object
        let mut array_prototype = HashMap::new();
        array_prototype.insert(
            "prototype".to_string(),
            RuntimeObject::Object(Rc::new(RefCell::new(JSObject {
                properties: HashMap::new(),
            }))),
        );
        let array = RuntimeObject::Object(Rc::new(RefCell::new(JSObject {
            properties: array_prototype,
        })));
        scope.insert("Array".to_string(), array);

        Global { scope }
    }

    pub fn get(&self, key: &str) -> Option<&RuntimeObject> {
        self.scope.get(key)
    }

    pub fn set(&mut self, name: &str, ob: RuntimeObject) {
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
    pub fn run(&mut self) -> Result<RuntimeObject, Error> {
        let mut ev = Evaluator::new(self.context);
        ev.eval(&self.ast)
    }
}
