use futures_core::Future;
use unknownrori_simple_thread_pool::crossbeam_channel::unbounded;

use crate::{executor::Executor, spawner::Spawner};

pub struct Runtime {
    spawner: Option<Spawner>,
    executor: Executor,
}

impl Runtime {
    pub fn new() -> Self {
        let (task_sender, ready_queue) = unbounded();

        let spawner = Spawner::new(task_sender);
        let executor = Executor::new(ready_queue, 4);

        Self {
            spawner: Some(spawner),
            executor,
        }
    }

    pub fn spawn(&self, f: impl Future<Output = ()> + 'static + Send) {
        self.spawner.as_ref().unwrap().spawn(f);
    }

    pub fn run(&mut self) {
        let spawner = self.spawner.take().unwrap();
        drop(spawner);

        self.executor.run();
    }
}
