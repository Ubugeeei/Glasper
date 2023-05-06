#![allow(dead_code)]

use crate::engine::execution::vm::VirtualMachine;

use super::constant::PROTOTYPE_KEY_NAME;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ptr::NonNull,
};

#[derive(Debug)]
pub struct JSObject {
    pub(crate) properties: HashMap<String, &'static mut JSObject>,
    pub(crate) _type: JSType,
}

/// static impl
impl JSObject {
    pub(crate) fn new() -> Self {
        JSObject {
            properties: HashMap::new(),
            _type: JSType::Object,
        }
    }

    pub(crate) fn from_raw_ptr<'a>(ptr: i64) -> &'a JSObject {
        let ptr = NonNull::new(ptr as *mut JSObject).unwrap();
        unsafe { ptr.as_ref() }
    }

    pub(crate) fn from_raw_ptr_mut<'a>(ptr: i64) -> &'a mut JSObject {
        let mut ptr = NonNull::new(ptr as *mut JSObject).unwrap();
        unsafe { ptr.as_mut() }
    }
}

/// core impl
impl JSObject {
    pub(crate) fn as_raw_ptr(&self) -> i64 {
        self as *const JSObject as i64
    }

    pub(crate) fn get(&self, key: &str) -> Option<&JSObject> {
        if let Some(prop) = self.properties.get(key) {
            Some(prop)
        } else {
            self.recursive_follow_prototype(key)
        }
    }

    fn recursive_follow_prototype(&self, key: &str) -> Option<&JSObject> {
        if let Some(prop) = self.properties.get(key) {
            Some(prop)
        } else if let Some(prototype) = self.properties.get(PROTOTYPE_KEY_NAME) {
            prototype.recursive_follow_prototype(key)
        } else {
            None
        }
    }
}

impl Display for JSObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self._type {
            JSType::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b),
            JSType::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n),
            JSType::String(s) => write!(f, "\x1b[32m'{}'\x1b[0m", s),
            JSType::Object => write!(f, "\x1b[34m[Object]\x1b[0m"),
            JSType::Array => write!(f, "\x1b[34m[Array]\x1b[0m"),
            JSType::Function => write!(f, "[Function]"),
            JSType::Undefined => write!(f, "\x1b[30mundefined\x1b[0m"),
            JSType::NativeFunction(_) => write!(f, "[native code]"),
        }
    }
}

pub(crate) enum JSType {
    Boolean(bool),
    Number(f64),
    String(String),
    Array,
    Object,
    Function,
    Undefined,
    NativeFunction(
        fn(vm: &mut VirtualMachine, this: &mut JSObject, _: Vec<JSObject>) -> &'static mut JSObject,
    ),
}

impl Debug for JSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSType::Boolean(b) => write!(f, "Boolean({})", b),
            JSType::Number(n) => write!(f, "Number({})", n),
            JSType::String(s) => write!(f, "String({})", s),
            JSType::Array => write!(f, "Array"),
            JSType::Object => write!(f, "Object"),
            JSType::Function => write!(f, "Function"),
            JSType::Undefined => write!(f, "Undefined"),
            JSType::NativeFunction(_) => write!(f, "NativeFunction"),
        }
    }
}
