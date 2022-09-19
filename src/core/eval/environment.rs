use std::collections::HashMap;

use super::object::Object;

pub struct Environment {
    pub store: HashMap<String, Variable>,
}
impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str, var: Variable) {
        self.store.insert(name.to_string(), var);
    }
}

#[derive(PartialEq, Debug)]
pub struct Variable {
    pub kind: VariableKind,
    pub value: Object,
}
impl Variable {
    pub fn new(kind: VariableKind, value: Object) -> Variable {
        Variable { kind, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableKind {
    Let,
    Const,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::eval::object::GNumber;

    #[test]
    fn test_set_get() {
        let mut env = Environment::new();
        env.set(
            "a",
            Variable::new(VariableKind::Const, Object::Number(GNumber::new(1.0))),
        );
        assert_eq!(
            env.get("a"),
            Some(&Variable::new(
                VariableKind::Const,
                Object::Number(GNumber::new(1.0))
            ))
        );
    }
}
