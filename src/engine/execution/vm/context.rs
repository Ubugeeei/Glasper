use std::{cell::RefCell, collections::HashMap, ptr::NonNull, rc::Rc};

use crate::engine::execution::objects::js_objects::JSObject;

pub(crate) struct ExecutionContext {
    // this: JSObject,
    pub(crate) context: Rc<RefCell<Context>>,
}
impl ExecutionContext {
    pub(crate) fn new() -> Self {
        ExecutionContext {
            context: Rc::new(RefCell::new(Context::new(None))),
        }
    }

    pub(crate) fn scope_in(&mut self) {
        let new_context = Rc::new(RefCell::new(Context::new(Some(self.context.clone()))));
        self.context = new_context;
    }

    pub(crate) fn scope_out(&mut self) {
        let new_context = self.context.borrow().outer.clone().unwrap();
        self.context = new_context
    }
}

type ContextSlot = HashMap<String, NonNull<JSObject>>;
pub(crate) struct Context {
    slots: Rc<RefCell<ContextSlot>>,
    outer: Option<Rc<RefCell<Context>>>,
}
impl Context {
    fn new(outer: Option<Rc<RefCell<Context>>>) -> Self {
        Context {
            slots: Rc::new(RefCell::new(HashMap::new())),
            outer,
        }
    }

    pub(crate) fn set(&self, name: String, value: NonNull<JSObject>) {
        self.slots.borrow_mut().insert(name, value);
    }

    pub(crate) fn get(&self, name: &str) -> Option<NonNull<JSObject>> {
        self.slots.borrow().get(name).cloned().or_else(|| {
            self.outer
                .as_ref()
                .and_then(|parent| parent.borrow().get(name))
        })
    }
}
