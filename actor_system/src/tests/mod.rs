#[cfg(test)]
mod actor_system_tests {
    use crate::{ActorSystem, Actor, ActorRef, Message};
    use std::time::Duration;
    use tokio::time::sleep;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct WorkerActor {
        id: String,
        work_count: AtomicU32,
    }

    struct ManagerActor {
        id: String,
        total_work: AtomicU32,
    }

    #[derive(Debug)]
    struct Work(u32);

    #[derive(Debug)]
    struct WorkComplete(u32);

    impl Message for Work {}
    impl Message for WorkComplete {}

    #[async_trait::async_trait]
    impl Actor for WorkerActor {
        type Message = Work;

        fn id(&self) -> &str { &self.id }

        async fn handle(&mut self, msg: Self::Message) {
            let Work(amount) = msg;
            // 복잡한 계산 시뮬레이션
            let result = (0..amount).fold(0, |acc, x| acc + x * x);
            self.work_count.fetch_add(amount, Ordering::SeqCst);
            println!("Worker {} completed work: {} (total: {})", self.id, result, self.work_count.load(Ordering::SeqCst));
            sleep(Duration::from_millis(10)).await;
        }
    }

    #[async_trait::async_trait]
    impl Actor for ManagerActor {
        type Message = WorkComplete;

        fn id(&self) -> &str { &self.id }

        async fn handle(&mut self, msg: Self::Message) {
            let WorkComplete(amount) = msg;
            let total = self.total_work.fetch_add(amount, Ordering::SeqCst) + amount;
            println!("Manager: Total work done: {}", total);
        }
    }

    #[tokio::test]
    async fn test_complex_actor_system() {
        let system = ActorSystem::new();

        let manager = system.create_actor(ManagerActor { 
            id: "manager".into(), 
            total_work: AtomicU32::new(0),
        });

        let worker_count = 5;
        let mut workers = Vec::with_capacity(worker_count);
        for i in 0..worker_count {
            let worker = system.create_actor(WorkerActor { 
                id: format!("worker_{}", i), 
                work_count: AtomicU32::new(0),
            });
            workers.push(worker);
        }

        let work_iterations = 20;

        for i in 0..work_iterations {
            for (j, worker) in workers.iter().enumerate() {
                let work_amount = (i * worker_count + j) as u32 + 1;
                worker.send(Work(work_amount)).await.unwrap();
                manager.send(WorkComplete(work_amount)).await.unwrap();
            }
        }

        // 모든 작업이 처리될 때까지 잠시 대기
        sleep(Duration::from_secs(2)).await;

        // 시스템 종료
        system.shutdown().await;

        println!("Actor system shut down successfully");
    }
}