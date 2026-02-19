//! Timer utility for scheduled tasks

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

type TaskFn = Box<dyn Fn() + Send + Sync + 'static>;

pub struct Timer {
    tasks: HashMap<Duration, TaskFn>,
    cancel_senders: HashMap<Duration, mpsc::Sender<()>>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            cancel_senders: HashMap::new(),
        }
    }

    /// Register a task to run at the given interval
    pub fn register_task<F>(&mut self, task: F, interval_duration: Duration)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.tasks.insert(interval_duration, Box::new(task));
    }

    /// Start all registered tasks
    pub fn start(&mut self) {
        for (duration, task) in self.tasks.drain() {
            let (tx, mut rx) = mpsc::channel::<()>(1);
            self.cancel_senders.insert(duration, tx);

            tokio::spawn(async move {
                let mut ticker = interval(duration);
                // Run immediately first time
                task();
                
                loop {
                    tokio::select! {
                        _ = ticker.tick() => {
                            task();
                        }
                        _ = rx.recv() => {
                            tracing::info!("Task with interval {:?} stopped", duration);
                            break;
                        }
                    }
                }
            });
        }
    }

    /// Stop all running tasks
    pub async fn stop(&mut self) {
        for (duration, sender) in self.cancel_senders.drain() {
            let _ = sender.send(()).await;
            tracing::info!("Task with interval {:?} has been canceled", duration);
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
