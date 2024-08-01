use crate::message::Message;
use tokio::sync::mpsc;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Actor: Send + 'static {
    type Message: Message;

    fn id(&self) -> &str;
    async fn handle(&mut self, msg: Self::Message);
}

#[derive(Clone)]
pub struct ActorRef<A: Actor> {
    sender: Arc<mpsc::Sender<A::Message>>,
}

impl<A: Actor> ActorRef<A> {
    pub(crate) fn new(sender: mpsc::Sender<A::Message>) -> Self {
        ActorRef {
            sender: Arc::new(sender),
        }
    }

    pub async fn send(&self, msg: A::Message) -> Result<(), mpsc::error::SendError<A::Message>> {
        self.sender.send(msg).await
    }
}

pub(crate) struct ActorContainer<A: Actor> {
    pub actor: A,
    pub receiver: mpsc::Receiver<A::Message>,
}

impl<A: Actor> ActorContainer<A> {
    pub fn new(actor: A, receiver: mpsc::Receiver<A::Message>) -> Self {
        ActorContainer { actor, receiver }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.actor.handle(msg).await;
        }
    }
}