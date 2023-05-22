use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};
use alloc::boxed::Box;

pub mod simple_executor;

pub struct Task {
    // A task returns nothing, all we need is its effect.
    // stores a trait object in a Box, dynamically dispatching methods based on the Task type
    // Pin the Future: Store on heap and avoid &mut refs to it.
    future: Pin<Box<dyn Future<Output = ()>>> 
}

impl Task {

    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
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