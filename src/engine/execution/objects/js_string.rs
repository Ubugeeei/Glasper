#![allow(dead_code)]
use crate::engine::execution::vm::VirtualMachine;

use super::{js_object::JSType, object::Object};

pub(crate) struct JSString;

impl JSString {
    pub(crate) fn create<'a>(
        id: u32,
        allocated: &'a mut Object,
        vm: &mut VirtualMachine,
    ) -> &'a mut Object {
        let object_ref = allocated.as_js_object_mut();
        let s = vm.constant_table.get(id);
        object_ref._type = JSType::String(s.to_owned());

        let mut prototype = vm.heap.alloc().unwrap();

        let mut string_char_code_at_fn = vm.heap.alloc().unwrap();
        string_char_code_at_fn.as_js_object_mut()._type =
            JSType::NativeFunction(string_char_code_at);

        prototype
            .as_js_object_mut()
            .properties
            .insert("charCodeAt".to_string(), string_char_code_at_fn);

        object_ref
            .properties
            .insert("prototype".to_string(), prototype);

        allocated
    }
}

fn string_char_code_at(vm: &mut VirtualMachine, this: &mut Object, _: Vec<Object>) -> Object {
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
