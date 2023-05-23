use super::{
    Task,
    TaskId,
};
use alloc::{
    task::Wake,
    collections::BTreeMap,
    sync::Arc,
};
use core::task::{
    Context, 
    Poll,
    Waker,
};
use crossbeam_queue::ArrayQueue;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same Id already exist in tasks");
        }
        self.task_queue.push(task_id).expect("queue");
    }

    fn run_ready_tasks(&mut self) {
        // destructuring "self" to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        // loop over all tasks, remove the last, capturing the task_id
        while let Ok(task_id) = task_queue.pop() {
            // returns a any task(mut) that matches with the task_id
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            // create a waker for task, using the task_id
            let waker = waker_cache
                .entry(task_id)
                // Create waker, if it doesn't already exist in the waker_cache
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            // retrieve the task context and poll task
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue is full");
    }
    
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }
}

impl Wake for TaskWaker {
    // takes ownership -> increases ref count
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    // doesn't take ownership, requires only a reference
    // not all types support waking by reference .'. optional
    // * adds better perf, bc it eliminates unwanted ref count changes
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}