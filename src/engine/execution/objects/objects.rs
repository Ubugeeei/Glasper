#![allow(dead_code)]

use std::ptr::NonNull;

use super::js_object::JSObject;

pub struct Object {
    pub(crate) ptr: NonNull<JSObject>,
}

impl Object {
    pub(crate) fn new(ptr: NonNull<JSObject>) -> Self {
        Object { ptr }
    }

    pub(crate) fn from_row_ptr(ptr: usize) -> Self {
        Object {
            ptr: NonNull::new(ptr as *mut JSObject).unwrap(),
        }
    }

    pub(crate) fn as_js_object_ref(&self) -> &JSObject {
        unsafe { self.ptr.as_ref() }
    }

    pub(crate) fn as_js_object_mut(&mut self) -> &mut JSObject {
        unsafe { self.ptr.as_mut() }
    }
}
