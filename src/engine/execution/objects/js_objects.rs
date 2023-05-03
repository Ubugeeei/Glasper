use std::{marker::PhantomData, ptr::NonNull};

pub(crate) struct JSObject<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> JSObject<T> {
    pub(crate) fn new(ptr: NonNull<T>) -> Self {
        JSObject {
            ptr,
            _marker: PhantomData,
        }
    }
}
