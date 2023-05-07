#![allow(dead_code)]
use crate::engine::core::vm::VirtualMachine;

use super::{
    constant::PROTOTYPE_KEY_NAME,
    js_object::{JSObject, JSType},
};

pub(crate) struct JSString;

impl JSString {
    pub(crate) fn create<'a>(
        s: String,
        allocated: &'a mut JSObject,
        vm: &mut VirtualMachine,
    ) -> &'a mut JSObject {
        // string prototype
        let prototype = vm.heap.alloc().unwrap();
        let mut string_char_code_at_fn = vm.heap.alloc().unwrap();

        string_char_code_at_fn._type = JSType::NativeFunction(string_char_code_at);
        prototype
            .properties
            .insert("charCodeAt".to_string(), string_char_code_at_fn);

        // create string instance
        allocated._type = JSType::String(s);
        allocated
            .properties
            .insert(String::from(PROTOTYPE_KEY_NAME), prototype);

        allocated
    }
}

fn string_char_code_at(
    vm: &mut VirtualMachine,
    this: &mut JSObject,
    _: Vec<JSObject>,
) -> &'static mut JSObject {
    let n = match this._type {
        JSType::Number(n) => n,
        _ => panic!("TypeError: Number.prototype.toString is not generic"),
    };

    // TODO: string object
    let s = n.to_string();
    let mut string = vm.heap.alloc().unwrap();
    string._type = JSType::String(s);
    string
}
