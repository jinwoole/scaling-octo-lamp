use crate::{Actor, ActorSystem, Message};
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug)]
struct TestMessage(u32);

impl Message for TestMessage {}

struct TestActor {
    id: String,
    counter: Arc<AtomicU32>,
}

#[async_trait]
impl Actor for TestActor {
    type Message = TestMessage;

    fn id(&self) -> &str {
        &self.id
    }

    async fn handle(&mut self, msg: TestMessage) {
        self.counter.fetch_add(msg.0, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn test_create_actor_system() {
    let system = ActorSystem::new();
    assert!(Arc::strong_count(&system) == 1, "ActorSystem should be created with one strong reference");
}

#[tokio::test]
async fn test_create_actor() {
    let system = ActorSystem::new();
    let counter = Arc::new(AtomicU32::new(0));
    let actor = TestActor {
        id: "test_actor".to_string(),
        counter: counter.clone(),
    };
    let actor_ref = system.create_actor(actor);
    assert!(actor_ref.send(TestMessage(1)).await.is_ok(), "Should be able to send a message to the actor");
}

#[tokio::test]
async fn test_actor_message_handling() {
    let system = ActorSystem::new();
    let counter = Arc::new(AtomicU32::new(0));
    let actor = TestActor {
        id: "test_actor".to_string(),
        counter: counter.clone(),
    };
    let actor_ref = system.create_actor(actor);

    actor_ref.send(TestMessage(1)).await.expect("Failed to send message");
    actor_ref.send(TestMessage(2)).await.expect("Failed to send message");
    
    sleep(Duration::from_millis(50)).await;
    
    assert_eq!(counter.load(Ordering::SeqCst), 3, "Counter should be 3 after processing messages");
}

#[tokio::test]
async fn test_multiple_actors() {
    let system = ActorSystem::new();
    let counter1 = Arc::new(AtomicU32::new(0));
    let counter2 = Arc::new(AtomicU32::new(0));

    let actor1 = TestActor {
        id: "actor1".to_string(),
        counter: counter1.clone(),
    };
    let actor2 = TestActor {
        id: "actor2".to_string(),
        counter: counter2.clone(),
    };

    let actor_ref1 = system.create_actor(actor1);
    let actor_ref2 = system.create_actor(actor2);

    actor_ref1.send(TestMessage(1)).await.expect("Failed to send message to actor1");
    actor_ref2.send(TestMessage(2)).await.expect("Failed to send message to actor2");

    sleep(Duration::from_millis(50)).await;

    assert_eq!(counter1.load(Ordering::SeqCst), 1, "Counter1 should be 1");
    assert_eq!(counter2.load(Ordering::SeqCst), 2, "Counter2 should be 2");
}