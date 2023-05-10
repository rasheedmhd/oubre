use alloc::alloc::{
    Layout,
    GlobalAlloc,
};



use core::ptr::null_mut;
use super::Locked;

struct ListNode {
    // all nodes have the same fixed size
    next: Option<&'static mut ListNode>
}

unsafe impl GlobalAlloc for Locked<FSBAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match best_fit_index(&layout) {
            Some(index) => {
                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => {
                        // // no block exist, so we create a new block allocation
                        // let block_size =  BLOCK_SIZES[index];
                        // // only works on block size of power 2
                        // let block_align = block_size;

                        let (size, align) = (BLOCK_SIZES[index], BLOCK_SIZES[index]);
                        let layout = Layout::from_size_align(size, align)
                            .unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!();
    }
}


/// block sizes
/// must be a power of 2 to help with alignments (size alignments must be powers of 2)
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128,256, 512, 1024, 2048];

/// FSB -> [F]ixed[S]ized[B]lock 
pub struct FSBAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FSBAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        FSBAllocator { 
            list_heads: [ EMPTY; BLOCK_SIZES.len() ], 
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    } 

    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => null_mut(),
        }
    }

}

// Choose the best fitting block size for a given layout
// Returns an index into the BLOCK_SIZES array
// which is used as an index into the list_heads array
fn best_fit_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    // Returns an Option of the index 
    BLOCK_SIZES.iter().position( |&s| s >= required_block_size )
}