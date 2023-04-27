use alloc::alloc::{
    GlobalAlloc,
    Layout,
}

use core::ptr::{null_mut, null};

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut 8, _layout: Layout) {
        panic!("dealloc should never be called")
    }

}
