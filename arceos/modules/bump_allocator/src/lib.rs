#![no_std]

use core::alloc::Layout;
use core::ptr::NonNull;
use allocator::{AllocResult, AllocError, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    b_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            b_count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = self.start;
        self.p_pos = self.end;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let end = self.b_pos + layout.size();
        if end > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        let start_ptr = NonNull::new(self.b_pos as *mut u8).unwrap();
        self.b_pos = end;
        self.b_count += 1;
        Ok(start_ptr)
    }

    /// Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        if self.b_count == 0 {
            return;
        }
        self.b_count -= 1;
        if self.b_count == 0 {
            self.b_pos = self.start;
        }
    }

    /// Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    /// Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos  // not (total - used)
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    /// The size of a memory page.
    const PAGE_SIZE: usize = PAGE_SIZE;

    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, _align_pow2: usize) -> AllocResult<usize> {
        let required = num_pages * Self::PAGE_SIZE;
        if required > self.p_pos - self.b_pos {
            return Err(AllocError::NoMemory);
        }
        self.p_pos -= required;
        Ok(self.p_pos)
    }

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        unimplemented!()
    }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        (self.end - self.start) / Self::PAGE_SIZE
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / Self::PAGE_SIZE
    }
}