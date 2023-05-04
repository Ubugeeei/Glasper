#![allow(dead_code)]
use crate::engine::execution::vm::vm::VM;

use super::{js_object::JSType, objects::Object};

pub(crate) struct JSNumber;

impl JSNumber {
    pub(crate) fn create<'a>(n: f64, allocated: &'a mut Object, vm: &mut VM) -> &'a mut Object {
        let object_ref = allocated.as_js_object_mut();
        object_ref._type = JSType::Number(n);

        let mut prototype = vm.heap.alloc().unwrap();

        let mut to_string_fn = vm.heap.alloc().unwrap();
        to_string_fn.as_js_object_mut()._type = JSType::NativeFunction(number_to_string);

        prototype
            .as_js_object_mut()
            .properties
            .insert("toString".to_string(), to_string_fn);

        object_ref
            .properties
            .insert("prototype".to_string(), prototype);

        allocated
    }
}

fn number_to_string(vm: &mut VM, this: &mut Object, _: Vec<Object>) -> Object {
    let n = match this.as_js_object_mut()._type {
        JSType::Number(n) => n,
        _ => panic!("TypeError: Number.prototype.toString is not generic"),
    };

    // TODO: string object
    let s = n.to_string();
    let mut string = vm.heap.alloc().unwrap();
    string.as_js_object_mut()._type = JSType::String(s);
    string
}
