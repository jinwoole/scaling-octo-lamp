use crate::actor::{Actor, ActorContainer};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

// 실행 구조체
pub(crate) struct Executor {
    runtime: Runtime,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new().unwrap(),
        }
    }

    // 액터 실행 메서드
    pub fn spawn<A: Actor>(&self, mut container: ActorContainer<A>) -> JoinHandle<()> {
        self.runtime.spawn(async move {
            container.run().await;
        })
    }

    // 실행기 종료 메서드
    pub async fn shutdown(self) {
        self.runtime.shutdown_background();
    }
}