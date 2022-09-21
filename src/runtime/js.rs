use std::io::Error;

use crate::engine::{
    api::{Context, Isolate, Script},
    eval::object::{GBuiltinFunction, Object},
    handle_scope::HandleScope,
};

use super::binding::logger::console_log;

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
        let handle_scope = HandleScope::new();
        let mut context = Context::new(handle_scope);

        let global = context.global();
        let logger = Object::BuiltinFunction(GBuiltinFunction::new("console_log", console_log));
        global.set("console_log", logger);

        Self {
            isolate: Isolate::new(context),
        }
    }

    pub fn execute(&mut self, source: String) -> Result<Object, Error> {
        let scope = self.get_cxt();
        let mut script = Script::compile(source, scope);
        script.run()
    }

    fn get_cxt(&mut self) -> &mut Context {
        &mut self.isolate.context
    }
}
