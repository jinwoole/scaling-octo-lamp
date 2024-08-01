use actor_system::{ActorSystem, Actor, Message};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::sync::atomic::{AtomicU64, Ordering};
use rand::Rng;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Work(u64);

#[derive(Debug, Clone)]
struct WorkComplete(u64);

#[derive(Debug, Clone)]
struct GetWorkDone;

impl Message for Work {}
impl Message for WorkComplete {}
impl Message for GetWorkDone {}

struct WorkerActor {
    id: String,
    work_done: AtomicU64,
}

struct ManagerActor {
    id: String,
    total_work: AtomicU64,
}

#[async_trait::async_trait]
impl Actor for WorkerActor {
    type Message = Work;

    fn id(&self) -> &str {
        &self.id
    }

    async fn handle(&mut self, msg: Self::Message) {
        match msg {
            Work(amount) => {
                let result = (0..amount).fold(0u64, |acc, x| acc.wrapping_add(x.wrapping_mul(x)));
                self.work_done.fetch_add(amount, Ordering::SeqCst);
                let sleep_duration = rand::thread_rng().gen_range(10..100);
                sleep(Duration::from_millis(sleep_duration)).await;
                println!("Worker {} completed work: {} (total: {})", self.id, result, self.work_done.load(Ordering::SeqCst));
            }
        }
    }
}

#[async_trait::async_trait]
impl Actor for ManagerActor {
    type Message = WorkComplete;

    fn id(&self) -> &str {
        &self.id
    }

    async fn handle(&mut self, msg: Self::Message) {
        match msg {
            WorkComplete(amount) => {
                let total = self.total_work.fetch_add(amount, Ordering::SeqCst) + amount;
                println!("Manager: Total work done: {}", total);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let system = ActorSystem::new();

    let worker_count = 10;
    let mut workers = Vec::with_capacity(worker_count);
    for i in 0..worker_count {
        let worker = system.create_actor(WorkerActor {
            id: format!("worker_{}", i),
            work_done: AtomicU64::new(0),
        });
        workers.push(Arc::new(worker));
    }

    let manager = Arc::new(system.create_actor(ManagerActor {
        id: "manager".to_string(),
        total_work: AtomicU64::new(0),
    }));

    let total_work_items = 10000;
    let max_work_amount = 1000;

    println!("Starting benchmark with {} work items...", total_work_items);
    let start_time = Instant::now();

    let mut handles = Vec::new();
    for _ in 0..total_work_items {
        let worker = Arc::clone(&workers[rand::thread_rng().gen_range(0..worker_count)]);
        let manager = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let work_amount = rand::thread_rng().gen_range(1..=max_work_amount);
            let _ = worker.send(Work(work_amount)).await.unwrap();
            let sleep_duration = rand::thread_rng().gen_range(1..10);
            sleep(Duration::from_millis(sleep_duration)).await;
            let _ = manager.send(WorkComplete(work_amount)).await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start_time.elapsed();
    println!("Benchmark completed in {:?}", duration);

    sleep(Duration::from_secs(1)).await;

    println!("Work distribution among workers:");
    for (i, worker) in workers.iter().enumerate() {
        let work_done = worker.send(Work(0)).await.unwrap();
        println!("  Worker {}: {:?} units", i, work_done);
    }

    system.shutdown().await;
    println!("Actor system shut down successfully");
}