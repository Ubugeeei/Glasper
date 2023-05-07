use crate::{
    engine::core::host::{
        api::{Context, Isolate, Script},
        handles::HandleScope,
    },
    runtime::interface::JSRuntime,
};

use super::binding::console::ConsoleBuilder;

pub struct HostJSRuntime {
    isolate: Isolate,
}

impl Default for HostJSRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl HostJSRuntime {
    pub fn new() -> Self {
        let handle_scope = HandleScope::new();
        let mut context = Context::new(handle_scope);
        let global = context.global();

        // binding
        let console_builder = ConsoleBuilder::new();
        let console = console_builder.build();
        global.set("console", console);

        let mut isolate = Isolate::new(context);
        isolate.install_functions(vec!["src/runtime/host/array.js"]);

        HostJSRuntime { isolate }
    }
}

impl JSRuntime for HostJSRuntime {
    fn run(&mut self, source: String) {
        let scope = self.get_cxt();
        let mut script = Script::compile(source, scope);
        match script.run() {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
    }
}

impl HostJSRuntime {
    fn get_cxt(&mut self) -> &mut Context {
        &mut self.isolate.context
    }
}
