use std::collections::HashMap;

use crate::engine::eval::object::{GBuiltinFunction, GObject, GUndefined, Object};

pub struct ConsoleBuilder;
impl ConsoleBuilder {
    pub fn build() -> Object {
        let mut properties = HashMap::new();
        properties.insert(
            String::from("log"),
            Object::BuiltinFunction(GBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("debug"),
            Object::BuiltinFunction(GBuiltinFunction::new("log", log)),
        );
        properties.insert(
            String::from("warn"),
            Object::BuiltinFunction(GBuiltinFunction::new("log", log)),
        );

        Object::Object(GObject { properties })
    }
}

fn log(args: Vec<Object>) -> Object {
    for arg in args {
        print!("{}", arg);
        print!("\x20");
    }
    println!();

    Object::Undefined(GUndefined)
}
