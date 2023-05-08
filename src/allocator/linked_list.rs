use super::{
    align_up,
    Locked,
};

use core::{
    ptr::null_mut, 
    mem,
};
use alloc::alloc::{
    Layout,
    GlobalAlloc,
};


struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // perform layout adjustments 
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }
            alloc_start as *mut u8
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // layout adjustments 
        let (size, _) = LinkedListAllocator::size_align(layout);

        self.lock().add_free_region(ptr as usize, size)
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    //// Creates an empty LinkedListAllocator
   pub const fn new() -> Self {
        LinkedListAllocator { 
            head: ListNode::new(0) 
        }
    }
    /// Initialize the allocator with the given heap bounds.
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    /// Adds the given memory region to the front of the list.
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {

        // The freed region should be capable of holding a ListNode
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());
        //
        // create and append a ListNode at the start of the linkedlist
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr)
    }

    /// Looks for a free mem region with the given size and alignment and removes it from the list
    /// Returns the list node and the start address of the allocated mem
    fn find_region(&mut self, size: usize, align: usize) 
    -> Option<(&'static mut ListNode, usize)>
    {
        // reference to current list node, updated on each iteration
        let mut current_node = &mut self.head;
        // searching for large suitable mem region in the linkedlist
        while let Some(ref mut region) = current_node.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                // suitable region for removal
                let next = region.next.take();
                let ret = Some((current_node.next.take().unwrap(), alloc_start));
                current_node.next = next;
                return ret;
            } else {
                current_node = current_node.next.as_mut().unwrap();
            }
        }
        None
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) 
    -> Result<usize, ()>
    {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            // region too small -> memory overflow
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // rest of region too small to hold ListNode.
            return Err(());
        }

        // region suitable for allocation
        Ok(alloc_start)
    }
}