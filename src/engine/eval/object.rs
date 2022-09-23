#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use crate::engine::parse::ast::{BlockStatement, FunctionParameter};

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeObject {
    Boolean(JSBoolean),
    Number(JSNumber),
    String(JSString),
    RuntimeObject(JSObject),
    Function(JSFunction),
    BuiltinFunction(JSBuiltinFunction),
    Null(JSNull),
    Undefined(JSUndefined),
    NaN(JSNaN),
    Return(Box<RuntimeObject>),
}

impl RuntimeObject {
    pub fn get_type(&self) -> String {
        match self {
            Self::Boolean(_) => "boolean".to_string(),
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::RuntimeObject(_) => "object".to_string(),
            Self::Function(_) => "function".to_string(),
            Self::Null(_) => "object".to_string(),
            Self::Undefined(_) => "undefined".to_string(),
            Self::NaN(_) => "number".to_string(),
            Self::BuiltinFunction(_) => "function".to_string(),
            Self::Return(_) => "".to_string(),
        }
    }
}
impl Display for RuntimeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b.value),
            Self::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n.value),
            // Self::String(s) => write!(f, "\x1b[32m'{}'\x1b[0m", s.value),
            Self::String(s) => write!(f, "{}", s.value),

            Self::RuntimeObject(_) => write!(f, "\x1b[34m[RuntimeObject]\x1b[0m"),

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
pub struct JSNumber {
    pub value: f64,
}
impl JSNumber {
    pub fn into(o: RuntimeObject) -> RuntimeObject {
        match o {
            RuntimeObject::Number(n) => RuntimeObject::Number(n),
            RuntimeObject::Boolean(JSBoolean { value }) => RuntimeObject::Number(JSNumber {
                value: if value { 1.0 } else { 0.0 },
            }),
            RuntimeObject::String(s) => {
                let value = s.value.parse::<f64>();
                match value {
                    Ok(v) => RuntimeObject::Number(JSNumber { value: v }),
                    Err(_) => RuntimeObject::NaN(JSNaN {}),
                }
            }
            RuntimeObject::Null(_) => RuntimeObject::Number(JSNumber { value: 0.0 }),
            _ => RuntimeObject::NaN(JSNaN),
        }
    }
}
// TODO: impl prototype
impl JSNumber {
    pub fn new(value: f64) -> JSNumber {
        JSNumber { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSString {
    pub value: String,
}
// TODO: impl prototype
impl JSString {
    pub fn into(o: RuntimeObject) -> RuntimeObject {
        match o {
            RuntimeObject::String(s) => RuntimeObject::String(s),
            RuntimeObject::Number(n) => RuntimeObject::String(JSString {
                value: n.value.to_string(),
            }),
            RuntimeObject::Boolean(JSBoolean { value }) => RuntimeObject::String(JSString {
                value: value.to_string(),
            }),
            RuntimeObject::Null(_) => RuntimeObject::String(JSString {
                value: "null".to_string(),
            }),
            RuntimeObject::Undefined(_) => RuntimeObject::String(JSString {
                value: "undefined".to_string(),
            }),
            // TODO: literal
            _ => RuntimeObject::String(JSString {
                value: "".to_string(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSBoolean {
    pub value: bool,
}
impl JSBoolean {
    pub fn into(o: RuntimeObject) -> RuntimeObject {
        match o {
            RuntimeObject::Boolean(b) => RuntimeObject::Boolean(b),
            RuntimeObject::Number(JSNumber { value }) => RuntimeObject::Boolean(JSBoolean {
                value: value != 0.0,
            }),
            RuntimeObject::String(JSString { value }) => RuntimeObject::Boolean(JSBoolean {
                value: value != *"",
            }),
            RuntimeObject::Null(_) | RuntimeObject::Undefined(_) | RuntimeObject::NaN(_) => {
                RuntimeObject::Boolean(JSBoolean { value: false })
            }
            _ => RuntimeObject::Boolean(JSBoolean { value: true }),
        }
    }
}

// TODO: impl prototype
impl JSBoolean {
    pub fn new(value: bool) -> JSBoolean {
        JSBoolean { value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSObject {
    pub properties: HashMap<String, RuntimeObject>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct JSFunction {
    pub parameters: Vec<FunctionParameter>,
    pub body: BlockStatement,
}
impl JSFunction {
    pub fn new(parameters: Vec<FunctionParameter>, body: BlockStatement) -> JSFunction {
        JSFunction { parameters, body }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSNull;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSUndefined;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSNaN;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JSBuiltinFunction {
    name: String,
    pub func: fn(Vec<RuntimeObject>) -> RuntimeObject,
}
impl JSBuiltinFunction {
    pub fn new(name: &str, func: fn(Vec<RuntimeObject>) -> RuntimeObject) -> JSBuiltinFunction {
        JSBuiltinFunction {
            name: name.to_string(),
            func,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_into_number() {
        let case = vec![
            (
                RuntimeObject::Number(JSNumber { value: 1.0 }),
                RuntimeObject::Number(JSNumber { value: 1.0 }),
            ),
            (
                RuntimeObject::Boolean(JSBoolean { value: true }),
                RuntimeObject::Number(JSNumber { value: 1.0 }),
            ),
            (
                RuntimeObject::Boolean(JSBoolean { value: false }),
                RuntimeObject::Number(JSNumber { value: 0.0 }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "1".to_string(),
                }),
                RuntimeObject::Number(JSNumber { value: 1.0 }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "1.0".to_string(),
                }),
                RuntimeObject::Number(JSNumber { value: 1.0 }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "1.1".to_string(),
                }),
                RuntimeObject::Number(JSNumber { value: 1.1 }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "a".to_string(),
                }),
                RuntimeObject::NaN(JSNaN {}),
            ),
            (
                RuntimeObject::Null(JSNull {}),
                RuntimeObject::Number(JSNumber { value: 0.0 }),
            ),
            (
                RuntimeObject::Undefined(JSUndefined {}),
                RuntimeObject::NaN(JSNaN {}),
            ),
        ];

        for (input, expected) in case {
            assert_eq!(JSNumber::into(input), expected);
        }
    }

    #[test]
    fn test_into_boolean() {
        let case = vec![
            (
                RuntimeObject::Number(JSNumber { value: 1.0 }),
                RuntimeObject::Boolean(JSBoolean { value: true }),
            ),
            (
                RuntimeObject::Number(JSNumber { value: 0.0 }),
                RuntimeObject::Boolean(JSBoolean { value: false }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "1".to_string(),
                }),
                RuntimeObject::Boolean(JSBoolean { value: true }),
            ),
            (
                RuntimeObject::String(JSString {
                    value: "".to_string(),
                }),
                RuntimeObject::Boolean(JSBoolean { value: false }),
            ),
            (
                RuntimeObject::Null(JSNull {}),
                RuntimeObject::Boolean(JSBoolean { value: false }),
            ),
            (
                RuntimeObject::Undefined(JSUndefined {}),
                RuntimeObject::Boolean(JSBoolean { value: false }),
            ),
            (
                RuntimeObject::NaN(JSNaN {}),
                RuntimeObject::Boolean(JSBoolean { value: false }),
            ),
        ];

        for (input, expected) in case {
            assert_eq!(JSBoolean::into(input), expected);
        }
    }

    #[test]
    fn test_into_string() {
        let case = vec![
            (
                RuntimeObject::Number(JSNumber { value: 1.0 }),
                RuntimeObject::String(JSString {
                    value: "1".to_string(),
                }),
            ),
            (
                RuntimeObject::Boolean(JSBoolean { value: true }),
                RuntimeObject::String(JSString {
                    value: "true".to_string(),
                }),
            ),
            (
                RuntimeObject::Boolean(JSBoolean { value: false }),
                RuntimeObject::String(JSString {
                    value: "false".to_string(),
                }),
            ),
            (
                RuntimeObject::Null(JSNull {}),
                RuntimeObject::String(JSString {
                    value: "null".to_string(),
                }),
            ),
            (
                RuntimeObject::Undefined(JSUndefined {}),
                RuntimeObject::String(JSString {
                    value: "undefined".to_string(),
                }),
            ),
            (
                RuntimeObject::NaN(JSNaN {}),
                RuntimeObject::String(JSString {
                    value: "".to_string(),
                }),
            ),
        ];

        for (input, expected) in case {
            assert_eq!(JSString::into(input), expected);
        }
    }
}
