#![allow(dead_code)]

use std::fmt::Display;

use crate::engine::parse::ast::{BlockStatement, FunctionParameter};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Number(GNumber),
    Boolean(GBoolean),
    Function(GFunction),
    BuiltinFunction(GBuiltinFunction),
    Null(GNull),
    Undefined(GUndefined),
}

impl Object {
    fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::Function(_) => "function".to_string(),
            Self::Null(_) => "object".to_string(),
            Self::Undefined(_) => "undefined".to_string(),
            Self::BuiltinFunction(_) => "function".to_string(),
        }
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n.value),
            Self::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b.value),
            Self::Function(_) => write!(f, "[Function]"),
            Self::Null(_) => write!(f, "null"),
            Self::Undefined(_) => write!(f, "\x1b[30mundefined\x1b[0m"),
            Self::BuiltinFunction(b) => {
                write!(f, "\x1b[33mf\x1b[0m {}() {{ [native code] }}", b.name)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GNumber {
    pub value: f64,
}
// TODO: impl prototype
impl GNumber {
    pub fn new(value: f64) -> GNumber {
        GNumber { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GBoolean {
    pub value: bool,
}
// TODO: impl prototype
impl GBoolean {
    pub fn new(value: bool) -> GBoolean {
        GBoolean { value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GFunction {
    pub parameters: Vec<FunctionParameter>,
    pub body: BlockStatement,
}
impl GFunction {
    pub fn new(parameters: Vec<FunctionParameter>, body: BlockStatement) -> GFunction {
        GFunction { parameters, body }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GNull;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GUndefined;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GBuiltinFunction {
    name: String,
    pub func: fn(Vec<Object>) -> Object,
}
impl GBuiltinFunction {
    pub fn new(name: &str, func: fn(Vec<Object>) -> Object) -> GBuiltinFunction {
        GBuiltinFunction {
            name: name.to_string(),
            func,
        }
    }
}
