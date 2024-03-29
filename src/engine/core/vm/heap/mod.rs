use self::{gc::GarbageCollector, page_allocator::PageAllocator, virtual_memory::VirtualMemory};
use crate::engine::core::vm::objects::js_object::JSObject;

mod gc;
mod page_allocator;
mod virtual_memory;

pub(crate) struct Heap {
    virtual_memory: VirtualMemory,
    gc: GarbageCollector,
    next: *mut u8,
    end: *mut u8,
}

impl Heap {
    pub(crate) fn new(size: usize) -> Self {
        let page_allocator = Box::new(PageAllocator);
        let virtual_memory = VirtualMemory::new(page_allocator, size);
        let next = virtual_memory.address().unwrap().as_ptr();
        let end = unsafe { next.add(size) };
        Heap {
            virtual_memory,
            gc: GarbageCollector::new(),
            next,
            end,
        }
    }

    pub(crate) fn alloc(&mut self) -> Option<&'static mut JSObject> {
        let size = std::mem::size_of::<JSObject>();
        let align = std::mem::align_of::<JSObject>();
        let aligned_next = (self.next as usize + align - 1) & !(align - 1);
        let next = aligned_next as *mut u8;

        if next.wrapping_add(size) <= self.end {
            let ptr = next as *mut JSObject;
            let js_obj = JSObject::new();
            unsafe {
                ptr.write(js_obj);
                self.next = ptr.add(1) as *mut u8;
            }
            Some(JSObject::from_raw_ptr_mut(ptr as i64))
        } else {
            None
        }
    }

    // TODO: collect
    // pub(crate) fn free(&mut self) {
    //     self.virtual_memory.free();
    // }
}
