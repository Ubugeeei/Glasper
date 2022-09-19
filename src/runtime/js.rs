use std::io::Error;

use crate::engine::{
    api::{Context, Isolate, Script},
    eval::{environment::Environment, object::Object},
};

pub struct JavaScriptRuntime {
    isolate: Isolate,
}
impl Default for JavaScriptRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl JavaScriptRuntime {
    pub fn new() -> Self {
        let scope = Environment::new();
        let context = Context::new(scope);
        let isolate = Isolate::new(context);
        Self { isolate }
    }

    pub fn execute(&mut self, source: String) -> Result<Object, Error> {
        let scope = self.get_scope();
        let mut script = Script::compile(source, scope);
        script.run()
    }

    fn get_scope(&mut self) -> &mut Environment {
        &mut self.isolate.context.scope
    }
}
