#![allow(dead_code)]

use super::object::Object;
use crate::engine::execution::vm::vm::VM;
use std::{collections::HashMap, fmt::Display};

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
    NativeFunction(fn(vm: &mut VM, this: &mut Object, _: Vec<Object>) -> Object),
}
