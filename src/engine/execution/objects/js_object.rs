#![allow(dead_code)]

use crate::engine::execution::vm::VirtualMachine;

use super::{constant::PROTOTYPE_KEY_NAME, object::Object};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct JSObject {
    pub(crate) properties: HashMap<String, Object>,
    pub(crate) _type: JSType,
}

impl JSObject {
    pub(crate) fn new() -> Self {
        JSObject {
            properties: HashMap::new(),
            _type: JSType::Object,
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Object> {
        if let Some(prop) = self.properties.get(key) {
            Some(prop)
        } else {
            self.recursive_follow_prototype(key)
        }
    }

    fn recursive_follow_prototype(&self, key: &str) -> Option<&Object> {
        if let Some(prop) = self.properties.get(key) {
            Some(prop)
        } else if let Some(prototype) = self.properties.get(PROTOTYPE_KEY_NAME) {
            prototype.as_js_object_ref().recursive_follow_prototype(key)
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
            JSType::Array(_) => write!(f, "\x1b[34m[Array]\x1b[0m"),
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
    Array(Box<Object>),
    Object,
    Function,
    Undefined,
    NativeFunction(fn(vm: &mut VirtualMachine, this: &mut Object, _: Vec<Object>) -> Object),
}

impl Debug for JSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSType::Boolean(b) => write!(f, "Boolean({})", b),
            JSType::Number(n) => write!(f, "Number({})", n),
            JSType::String(s) => write!(f, "String({})", s),
            JSType::Array(_) => write!(f, "Array"),
            JSType::Object => write!(f, "Object"),
            JSType::Function => write!(f, "Function"),
            JSType::Undefined => write!(f, "Undefined"),
            JSType::NativeFunction(_) => write!(f, "NativeFunction"),
        }
    }
}
