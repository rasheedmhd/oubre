// sync init primitives, useful for lazy and one time init of static variables
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;
use futures_util::stream::Stream;

use core::{
    pin::Pin,
    task::{
        Poll,
        Context,
    }
};

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

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        // try init the SCANCODE_QUEUE panic if it has been init already.
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
        .expect("ScancodeStream::new should only be called once");
        ScancodeStream {
            _private: ()
        }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        match queue.pop() {
            Ok(scancode) => Poll::Ready(Some(scancode)),
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}