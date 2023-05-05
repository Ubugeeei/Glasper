#![allow(dead_code)]

use super::{constant::PROTOTYPE_KEY_NAME, js_object::JSType, object::Object};
use crate::engine::execution::vm::VirtualMachine;

pub(crate) struct JSNumber;

impl JSNumber {
    pub(crate) fn create<'a>(
        n: f64,
        allocated: &'a mut Object,
        vm: &mut VirtualMachine,
    ) -> &'a mut Object {
        // number prototype
        let mut prototype = vm.heap.alloc().unwrap();
        let mut to_string_fn = vm.heap.alloc().unwrap();
        to_string_fn.as_js_object_mut()._type = JSType::NativeFunction(number_to_string);
        prototype
            .as_js_object_mut()
            .properties
            .insert("toString".to_string(), to_string_fn);

        // create number instance
        let object_ref = allocated.as_js_object_mut();
        object_ref._type = JSType::Number(n);
        object_ref
            .properties
            .insert(String::from(PROTOTYPE_KEY_NAME), prototype);

        allocated
    }
}

fn number_to_string(vm: &mut VirtualMachine, this: &mut Object, _: Vec<Object>) -> Object {
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
