/// Allocators 
pub mod bump;
pub mod linked_list;
pub mod fixed_size_block;

use alloc::alloc::{
    GlobalAlloc,
    Layout,
};

use spin::{
    Mutex,
    MutexGuard,
};

use x86_64::{
    structures::paging::{
        mapper::MapToError,
        Mapper,
        Page,
        PageTableFlags,
        FrameAllocator,
        Size4KiB,
    },
    VirtAddr,
};

use core::ptr::null_mut;

use linked_list_allocator::LockedHeap;

use bump::BumpAllocator;
use linked_list::LinkedListAllocator;
use fixed_size_block::FSBAllocator;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

#[global_allocator]
static ALLOCATOR: Locked<FSBAllocator> = Locked::new(FSBAllocator::new());
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
// static ALLOCATOR: Dummy = Dummy; 
// static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub struct Dummy; 

unsafe impl GlobalAlloc for Dummy {

    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should never be called")
    }

}

pub fn init_heap<M, F>(
    mapper: &mut M,
    frame_allocator: &mut F
) -> Result<(), MapToError<Size4KiB>>
where
    M: Mapper<Size4KiB>,
    F: FrameAllocator<Size4KiB>,
{
    // creating a page range
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // mapping the pages of the page range
    for page in page_range {
        let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    };

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}


/// A wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address 'addr' upwards to alignment 'align'.
fn align_up(addr: usize, align: usize) -> usize {
    // ( addr + align - 1) & !(align -1)
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}