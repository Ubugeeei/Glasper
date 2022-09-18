#![allow(dead_code)]

use std::fmt::Display;

pub enum Object {
    Number(GNumber),
}

impl Object {
    fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
        }
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "\x1b[33m{}\x1b[0m", n.value),
        }
    }
}

pub struct GNumber {
    value: f64,
}
// TODO: impl prototype
impl GNumber {
    pub fn to_fixed(&self, precision: usize) -> f64 {
        format!("{:.1$}", self.value, precision).parse().unwrap()
    }
}
