use crate::actor::{Actor, ActorRef, ActorContainer};
use crate::executor::Executor;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::fmt;

pub struct ActorSystem {
    executor: Arc<Executor>,
}

impl fmt::Debug for ActorSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ActorSystem").finish()
    }
}

impl ActorSystem {
    pub fn new() -> Arc<Self> {
        Arc::new(ActorSystem {
            executor: Arc::new(Executor::new()),
        })
    }

    pub fn create_actor<A: Actor>(&self, actor: A) -> ActorRef<A> {
        let (sender, receiver) = mpsc::channel(100);
        let container = ActorContainer::new(actor, receiver);
        self.executor.spawn(container);
        ActorRef::new(sender)
    }
}

impl Clone for ActorSystem {
    fn clone(&self) -> Self {
        ActorSystem {
            executor: self.executor.clone(),
        }
    }
}