use crate::engine::eval::object::{GUndefined, Object};

// TODO: bind to context scope
pub struct Console;

impl Default for Console {
    fn default() -> Self {
        Console
    }
}

impl Console {
    #[allow(dead_code)]
    pub fn log(&self, args: Vec<Object>) -> Object {
        for arg in args {
            println!("{}", arg);
            println!("\x20");
        }

        Object::Undefined(GUndefined)
    }
}
