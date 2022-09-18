#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Number(GNumber),
    Boolean(GBoolean),
    Null(GNull),
    Undefined(GUndefined),
}

impl Object {
    fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::Null(_) => "object".to_string(),
            Self::Undefined(_) => "undefined".to_string(),
        }
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n.value),
            Self::Boolean(b) => write!(f, "\x1b[33m{}\x1b[0m", b.value),
            Self::Null(_) => write!(f, "null"),
            Self::Undefined(_) => write!(f, "\x1b[30mundefined\x1b[0m"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GNumber {
    value: i64,
}
// TODO: impl prototype
impl GNumber {}

#[derive(Debug, PartialEq, Eq)]
pub struct GBoolean {
    value: bool,
}
// TODO: impl prototype
impl GBoolean {}

#[derive(Debug, PartialEq, Eq)]
pub struct GNull;

#[derive(Debug, PartialEq, Eq)]
pub struct GUndefined;
