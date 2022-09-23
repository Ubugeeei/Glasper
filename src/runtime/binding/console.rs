use std::collections::HashMap;

use crate::engine::eval::object::{JSBuiltinFunction, JSObject, JSUndefined, RuntimeObject};

pub struct ConsoleBuilder;
impl Default for ConsoleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ConsoleBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> RuntimeObject {
        let mut properties = HashMap::new();
        properties.insert(
            String::from("log"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("debug"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("warn"),
            RuntimeObject::BuiltinFunction(JSBuiltinFunction::new("log", log)),
        );

        RuntimeObject::RuntimeObject(JSObject { properties })
    }
}

fn log(args: Vec<RuntimeObject>) -> RuntimeObject {
    for arg in args {
        print!("{}", arg);
        print!("\x20");
    }
    println!();

    RuntimeObject::Undefined(JSUndefined)
}
