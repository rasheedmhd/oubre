use super::Task;
use alloc::collections::VecDeque;

use core::task::{
    Waker,
    RawWaker, 
    Context,
    RawWakerVTable,
    Poll,
};

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor { 
            task_queue: VecDeque::new(), 
        }
    }

    // adds a new task to the task_queue at the end
    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    pub fn run(&mut self) {
        // check the task queue and remove the first task or None if task_queue is empty
        // capture the task and poll it
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            // create a context, remember that a context is a wrapper around a waker
            // so precede it with a waker instance's creation. 
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {} // task has been completed 
                // Task not completed yet, return task back to the task_queue 
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe {
        Waker::from_raw(dummy_raw_waker())
    }
}
