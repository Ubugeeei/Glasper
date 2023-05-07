use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::NonNull,
};

pub(super) struct PageAllocator;

impl PageAllocator {
    pub(super) fn allocate_pages(&self, num_pages: usize) -> Option<NonNull<u8>> {
        let layout = Layout::from_size_align(num_pages * 4096, 4096).unwrap();
        NonNull::new(unsafe { alloc(layout) })
    }

    pub(super) fn free_pages(&self, address: NonNull<u8>, num_pages: usize) {
        let layout = Layout::from_size_align(num_pages * 4096, 4096).unwrap();
        unsafe { dealloc(address.as_ptr(), layout) }
    }
}
