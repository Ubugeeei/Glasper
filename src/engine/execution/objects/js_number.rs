#![allow(dead_code)]
use crate::engine::execution::vm::vm::VM;

use super::js_objects::{JSObject, JSType};

pub(crate) struct JSNumber;

impl JSNumber {
    pub(crate) fn create<'a>(n: f64, allocated: &'a mut JSObject, vm: &mut VM) -> &'a mut JSObject {
        allocated._type = JSType::Number(n);
        let mut prototype = vm.heap.alloc().unwrap();

        let mut to_string_fn = vm.heap.alloc().unwrap();
        to_string_fn._type = JSType::NativeFunction(number_to_string);

        prototype
            .properties
            .insert("toString".to_string(), to_string_fn);

        allocated
            .properties
            .insert("prototype".to_string(), prototype);

        allocated
    }
}

fn number_to_string(vm: &mut VM, this: JSObject, _: Vec<JSObject>) -> JSObject {
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
