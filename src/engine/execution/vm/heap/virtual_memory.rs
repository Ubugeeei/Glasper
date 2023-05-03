use std::ptr::NonNull;

use super::page_allocator::PageAllocator;

pub(super) struct VirtualMemory {
    page_allocator: Box<PageAllocator>,
    address: Option<NonNull<u8>>,
    size: usize,
}

impl VirtualMemory {
    pub(super) fn new(page_allocator: Box<PageAllocator>, size: usize) -> Self {
        let num_pages = (size + 4095) / 4096;
        let address = page_allocator.allocate_pages(num_pages);
        VirtualMemory {
            page_allocator,
            address,
            size,
        }
    }

    pub(super) fn address(&self) -> Option<NonNull<u8>> {
        self.address
    }

    pub(super) fn size(&self) -> usize {
        self.size
    }

    pub(super) fn free(&mut self) {
        if let Some(address) = self.address {
            let num_pages = (self.size + 4095) / 4096;
            self.page_allocator.free_pages(address, num_pages);
        }
    }
}
