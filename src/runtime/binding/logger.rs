use crate::engine::eval::object::{GUndefined, Object};

pub fn console_log(args: Vec<Object>) -> Object {
    for arg in args {
        print!("{}", arg);
        print!("\x20");
    }
    println!();

    Object::Undefined(GUndefined)
}
