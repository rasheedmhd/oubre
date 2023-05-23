// sync init primitives, useful for lazy and one time init of static variables
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{
    print,
    println
};
use futures_util::{
    task::AtomicWaker,
    stream::{
        Stream,
        StreamExt
    },
};

use core::{
    pin::Pin,
    task::{
        Poll,
        Context,
    }
};

use pc_keyboard::{
    layouts,
    DecodedKey,
    HandleControl,
    Keyboard,
    ScancodeSet1,
};

// OnceCell, a cell type with interior mutability, that offers a read only access to the value
// that it wraps once initialized. It can only be written to once. 
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

static WAKER: AtomicWaker = AtomicWaker::new();

// called by the interrupt handler
// must not block or allocate
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
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

        // fast path, assuming the queue is not empty
        // avoids perf cost of registering the waker when queue not empty
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }
        // queue is potentially empty, register waker   
        // avoids missing the interrupt handler filling the queue
        // before second check. This guarantees that we get 
        // a wak eup for any scancode pushed after the first check
        WAKER.register(&cx.waker());

        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}