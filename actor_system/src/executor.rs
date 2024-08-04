use crate::actor::{Actor, ActorContainer};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub(crate) struct Executor {
    runtime: Runtime,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn spawn<A: Actor>(&self, mut container: ActorContainer<A>) -> JoinHandle<()> {
        self.runtime.spawn(async move {
            container.run().await;
        })
    }
}