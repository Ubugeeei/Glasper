use std::collections::HashMap;

use super::eval::object::Object;

pub struct HandleScope {
    pub store: HashMap<String, Variable>,
}
impl Default for HandleScope {
    fn default() -> Self {
        Self::new()
    }
}
impl HandleScope {
    pub fn new() -> HandleScope {
        HandleScope {
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VariableKind {
    Let,
    Const,
    Var,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::eval::object::GNumber;

    #[test]
    fn test_set_get() {
        let mut env = HandleScope::new();
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
