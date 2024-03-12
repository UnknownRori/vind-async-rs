use core::task::Context;

use alloc::sync::Arc;
use futures_task::waker_ref;
use unknownrori_simple_thread_pool::ThreadPool;

use crate::{channel::Receiver, task::Task};

pub type ReceiverTask = Receiver<Arc<Task>>;

/// Task executor that receives tasks off of a channel and runs them.
pub struct Executor {
    thread_pool: ThreadPool,
    ready_queue: ReceiverTask,
}

impl Executor {
    pub fn new(ready_queue: ReceiverTask, number_threads: usize) -> Self {
        let thread_pool = ThreadPool::new(number_threads).unwrap();
        Self {
            ready_queue,
            thread_pool,
        }
    }
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            self.thread_pool
                .execute(move || {
                    // Take the future, and if it has not yet completed (is still Some),
                    // poll it in an attempt to complete it.
                    let mut future_slot = task.future().lock();
                    if let Some(mut future) = future_slot.take() {
                        // Create a `LocalWaker` from the task itself
                        let waker = waker_ref(&task);
                        let context = &mut Context::from_waker(&waker);
                        // `BoxFuture<T>` is a type alias for
                        // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                        // We can get a `Pin<&mut dyn Future + Send + 'static>`
                        // from it by calling the `Pin::as_mut` method.
                        if future.as_mut().poll(context).is_pending() {
                            // We're not done processing the future, so put it
                            // back in its task to be run again in the future.
                            *future_slot = Some(future);
                        }
                    }
                })
                .unwrap();
        }
    }
}
