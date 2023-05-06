#![allow(dead_code)]

use std::ptr::NonNull;

use super::js_object::JSObject;

#[derive(Debug)]
pub struct Object {
    pub(crate) ptr: NonNull<JSObject>,
}

impl Object {
    pub(crate) fn new(ptr: NonNull<JSObject>) -> Self {
        Object { ptr }
    }

    pub(crate) fn from_row_ptr(ptr: i64) -> Self {
        Object {
            ptr: NonNull::new(ptr as *mut JSObject).unwrap(),
        }
    }

    pub(crate) fn raw_ptr(&self) -> i64 {
        self.ptr.as_ptr() as i64
    }

    pub(crate) fn as_js_object_ref(&self) -> &JSObject {
        unsafe { self.ptr.as_ref() }
    }

    pub(crate) fn as_js_object_mut(&mut self) -> &mut JSObject {
        unsafe { self.ptr.as_mut() }
    }
}
