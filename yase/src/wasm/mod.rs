use wasm_bindgen::prelude::*;
use std::{task::Wake, cell:RefCell};
use futures::task::waker_ref;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout(closure: JsValue, delay: u32);
}


thread_local! {
    pub EXECUTOR: Executor = Executor::new();
}

/// A task executor that works in a browser.
pub struct Executor {
    /// Queue of tasks ready to be polled
    queue: RefCell<Vec<Arc<Task>>>,
    waker: ExecutorWaker
}

impl Executor {
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(Vec::default()),
            waker: ExecutorWaker::new()
        }
    }

    /// Poll the tasks
    fn poll_tasks(&self) {
        while let Some(mut task) = self.queue.borrow_mut().pop() {
            if let Some(future) = task.future.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    task.future = Some(future);
                }
            }
        }

        *self.waker.is_scheduled.borrow_mut() = false;
    }

    /// Enqueue a task ready to be polled.
    /// Wake the executor
    fn enqueue(&self, task: Arc<Task>) {
        self.queue.borrow_mut().push(task);
        self.waker.wake();
    }
}

impl crate::executor::LocalSpawner for Executor {
    fn spawn<Fut: Future + 'static>(future: Fut) {
        Task::new(future);
    }
}

pub struct ExecutorWaker {
    /// The executor is scheduled to be run
    is_scheduled: RefCell<bool>
}

impl ExecutorWaker {
    pub fn new() -> Self {
        Self {
            is_scheduled: RefCel::new(false)
        }
    }

    pub fn wake(&self) {
        if self.is_scheduled.borrow() {
            return;
        }

        set_timeout(
            Closure::once_into_js(move || {
                Executor.poll_tasks();
            }),
            0,
        );
    }
}