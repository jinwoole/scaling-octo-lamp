use crate::actor::{Actor, ActorRef, ActorContainer};
use crate::executor::Executor;
use tokio::sync::mpsc;
use std::sync::Arc;

pub struct ActorSystem {
    executor: Arc<Executor>,
}

impl ActorSystem {
    pub fn new() -> Self {
        ActorSystem {
            executor: Arc::new(Executor::new()),
        }
    }

    pub fn create_actor<A: Actor>(&self, actor: A) -> ActorRef<A> {
        let (sender, receiver) = mpsc::channel(100);
        let container = ActorContainer::new(actor, receiver);
        self.executor.spawn(container);
        ActorRef::new(sender)
    }

    pub async fn shutdown(self) {
        match Arc::try_unwrap(self.executor) {
            Ok(executor) => executor.shutdown().await,
            Err(_) => println!("Failed to unwrap executor, it's still in use"),
        }
    }
}