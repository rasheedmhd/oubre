use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
};
use alloc::boxed::Box;

pub mod simple_executor;
pub mod keyboard;

pub struct Task {
    id: TaskId,
    // A task returns nothing, all we need is its effect.
    // stores a trait object in a Box, dynamically dispatching methods based on the Task type
    // Pin the Future: Store on heap and avoid &mut refs to it.
    future: Pin<Box<dyn Future<Output = ()>>> 
}

impl Task {

    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            // pin the future, and store it on the heap
            future: Box::pin(future),
        }
    }

    // should only be called by the executor
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        // convert self.future from Pin<Box<T>> to Pin<Box<&mut T>>
        self.future.as_mut().poll(context)
    }

}


// Executor with a Waker support,
// Letting the executor actually listens to wake calls, rather than polling 
// print_keypresses forever which consumes all of CPU 
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);


impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        // fetch_add atomically increases the value and the previous value
        // in one atomic op -> Calling TaskId::new in parallel doesn't cause
        // concurrency issues since each ID is returned only once
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}