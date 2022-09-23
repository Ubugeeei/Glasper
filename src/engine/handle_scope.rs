use std::collections::HashMap;

use super::eval::object::RuntimeObject;

pub struct HandleScope {
    pub scopes: Vec<HashMap<String, Variable>>,
}
impl Default for HandleScope {
    fn default() -> Self {
        Self::new()
    }
}
impl HandleScope {
    pub fn new() -> HandleScope {
        HandleScope {
            scopes: vec![
                HashMap::new(), // default scope
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            if let Some(variable) = scope.get(name) {
                return Some(variable);
            }
        }

        None
    }

    pub fn set(&mut self, name: &str, var: Variable) {
        let last_idx = self.scopes.len() - 1;
        self.scopes[last_idx].insert(name.to_string(), var);
    }

    pub fn assign(&mut self, name: &str, var: Variable) {
        let target_scope_idx = {
            let mut i = self.scopes.len() - 1;
            loop {
                if self.scopes[i].get(name).is_some() {
                    break i;
                }
                if i == 0 {
                    break self.scopes.len() - 1;
                }
                i -= 1;
            }
        };
        self.scopes[target_scope_idx].insert(name.to_string(), var);
    }

    pub fn scope_in(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn scope_out(&mut self) {
        self.scopes.pop();
    }
}

#[derive(PartialEq, Debug)]
pub struct Variable {
    pub kind: VariableKind,
    pub value: RuntimeObject,
}
impl Variable {
    pub fn new(kind: VariableKind, value: RuntimeObject) -> Variable {
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
    use crate::engine::eval::object::JSNumber;

    #[test]
    fn test_set_get() {
        let mut env = HandleScope::new();
        env.set(
            "a",
            Variable::new(
                VariableKind::Const,
                RuntimeObject::Number(JSNumber::new(1.0)),
            ),
        );
        assert_eq!(
            env.get("a"),
            Some(&Variable::new(
                VariableKind::Const,
                RuntimeObject::Number(JSNumber::new(1.0))
            ))
        );
    }
}
