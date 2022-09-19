// TODO: remove this
#![allow(dead_code)]

use std::collections::HashMap;

use crate::core::object::def::Object;

pub struct Environment {
    pub store: HashMap<String, Object>,
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

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.store.insert(name.to_string(), value);
    }
}

#[cfg(test)]
mod test {
    use crate::core::object::def::GNumber;

    use super::*;

    #[test]
    fn test_set_get() {
        let mut env = Environment::new();
        env.set("a", Object::Number(GNumber::new(1.0)));
        assert_eq!(env.get("a"), Some(Object::Number(GNumber::new(1.0))));
    }
}
