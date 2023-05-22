// sync init primitives, useful for lazy and one time init of static variables
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;

// OnceCell, a cell type with interior mutability, that offers a read only access to the value
// that it wraps once initialized. It can only be written to once. 
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

// called by the interrupt handler
// must not block or allocate
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}