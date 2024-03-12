use alloc::sync::Arc;
use vind_mutex::Mutex;

use futures_core::future::Future;

use crate::futures_ext::FutureExt;
use crate::task::{SenderTask, Task};

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
    task_sender: Option<SenderTask>,
}

impl Spawner {
    pub fn new(task_sender: SenderTask) -> Self {
        Self {
            task_sender: Some(task_sender),
        }
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();

        let task = Arc::new(Task::new(
            Mutex::new(Some(future)),
            self.task_sender.as_ref().unwrap().clone(),
        ));

        self.task_sender
            .as_ref()
            .unwrap()
            .send(task)
            .expect("too many tasks queued");
    }
}

impl Drop for Spawner {
    fn drop(&mut self) {
        drop(self.task_sender.take().unwrap());
    }
}
