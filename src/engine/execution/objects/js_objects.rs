#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display, ptr::NonNull};

pub(crate) struct JSObject<T> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) properties: HashMap<String, JSObject<T>>,
    _type: JSType,
}

impl<T> JSObject<T> {
    pub(crate) fn new(ptr: NonNull<T>) -> Self {
        JSObject {
            ptr,
            properties: HashMap::new(),
            _type: JSType::Object,
        }
    }
}

impl<T> Display for JSObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self._type {
            JSType::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b),
            JSType::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n),
            JSType::String(s) => write!(f, "\x1b[32m'{}'\x1b[0m", s),
            JSType::Object => write!(f, "\x1b[34m[Object]\x1b[0m"),
            JSType::Function => write!(f, "[Function]"),
            JSType::Undefined => write!(f, "\x1b[30mundefined\x1b[0m"),
        }
    }
}

pub(crate) enum JSType {
    Boolean(bool),
    Number(f64),
    String(String),
    Object,
    Function,
    Undefined,
}
