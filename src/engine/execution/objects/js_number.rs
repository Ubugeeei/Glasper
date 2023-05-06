#![allow(dead_code)]

use super::{
    constant::PROTOTYPE_KEY_NAME,
    js_object::{JSObject, JSType},
};
use crate::engine::execution::vm::VirtualMachine;

pub(crate) struct JSNumber;

impl JSNumber {
    pub(crate) fn create<'a>(
        n: f64,
        allocated: &'a mut JSObject,
        vm: &mut VirtualMachine,
    ) -> &'a mut JSObject {
        // number prototype
        let prototype = vm.heap.alloc().unwrap();
        let mut to_string_fn = vm.heap.alloc().unwrap();
        to_string_fn._type = JSType::NativeFunction(number_to_string);
        prototype
            .properties
            .insert("toString".to_string(), to_string_fn);

        // create number instance

        allocated._type = JSType::Number(n);
        allocated
            .properties
            .insert(String::from(PROTOTYPE_KEY_NAME), prototype);

        allocated
    }
}

fn number_to_string(
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
