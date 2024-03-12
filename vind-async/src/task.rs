use crate::channel::Sender;

use alloc::sync::Arc;
use futures_core::future::BoxFuture;
use futures_task::ArcWake;
use vind_mutex::Mutex;

/// In-progress future that should be pushed to completion.
pub type FutureFn = Mutex<Option<BoxFuture<'static, ()>>>;
pub type SenderTask = Sender<Arc<Task>>;

/// A future that can reschedule itself to be polled by an `Executor`.
pub struct Task {
    /// Future that will be pushed to completion
    future: FutureFn,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SenderTask,
}

impl Task {
    pub fn new(future: FutureFn, task_sender: SenderTask) -> Self {
        Self {
            future,
            task_sender,
        }
    }

    pub fn future(&self) -> &FutureFn {
        &self.future
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
