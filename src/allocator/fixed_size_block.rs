struct ListNode {
    // all nodes have thesame fixed size
    next: Option<&'static mut ListNode>
}


/// block sizes
/// must be a power of 2 to help with alignments (size alignments must be powers of 2)
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128,256, 5112, 1024, 2048];

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
}