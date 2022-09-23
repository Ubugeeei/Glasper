#![allow(dead_code)]

use std::fmt::Display;

use crate::engine::parse::ast::{BlockStatement, FunctionParameter};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Boolean(GBoolean),
    Number(GNumber),
    String(GString),
    Function(GFunction),
    BuiltinFunction(GBuiltinFunction),
    Null(GNull),
    Undefined(GUndefined),
    NaN(GNaN),
    Return(Box<Object>),
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Self::Boolean(_) => "boolean".to_string(),
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Function(_) => "function".to_string(),
            Self::Null(_) => "object".to_string(),
            Self::Undefined(_) => "undefined".to_string(),
            Self::NaN(_) => "number".to_string(),
            Self::BuiltinFunction(_) => "function".to_string(),
            Self::Return(_) => "".to_string(),
        }
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b.value),
            Self::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n.value),
            Self::String(s) => write!(f, "\x1b[32m'{}'\x1b[0m", s.value),

            Self::Function(_) => write!(f, "[Function]"),

            Self::Null(_) => write!(f, "null"),
            Self::Undefined(_) => write!(f, "\x1b[30mundefined\x1b[0m"),
            Self::NaN(_) => write!(f, "\x1b[33mNaN\x1b[0m"),

            Self::BuiltinFunction(b) => {
                write!(f, "\x1b[33mf\x1b[0m {}() {{ [native code] }}", b.name)
            }

            Self::Return(o) => write!(f, "{}", o),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GNumber {
    pub value: f64,
}
impl GNumber {
    pub fn into(o: Object) -> Object {
        match o {
            Object::Number(n) => Object::Number(n),
            Object::Boolean(GBoolean { value }) => Object::Number(GNumber {
                value: if value { 1.0 } else { 0.0 },
            }),
            Object::String(s) => {
                let value = s.value.parse::<f64>();
                match value {
                    Ok(v) => Object::Number(GNumber { value: v }),
                    Err(_) => Object::NaN(GNaN {}),
                }
            }
            Object::Null(_) => Object::Number(GNumber { value: 0.0 }),
            _ => Object::NaN(GNaN),
        }
    }
}
// TODO: impl prototype
impl GNumber {
    pub fn new(value: f64) -> GNumber {
        GNumber { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GString {
    pub value: String,
}
// TODO: impl prototype
impl GString {
    pub fn into(o: Object) -> Object {
        match o {
            Object::String(s) => Object::String(s),
            Object::Number(n) => Object::String(GString {
                value: n.value.to_string(),
            }),
            Object::Boolean(GBoolean { value }) => Object::String(GString {
                value: value.to_string(),
            }),
            Object::Null(_) => Object::String(GString {
                value: "null".to_string(),
            }),
            Object::Undefined(_) => Object::String(GString {
                value: "undefined".to_string(),
            }),
            // TODO: literal
            _ => Object::String(GString {
                value: "".to_string(),
            }),
        }
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
pub struct GNaN;

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
