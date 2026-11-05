//! Thread pool (Book ch.20, but rebuilt without peeking). Requirements:
//! - `execute` queues a job; workers pull from a shared Receiver behind Arc<Mutex<_>>.
//! - Dropping the pool joins all workers (graceful shutdown).
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    // workers, sender, ...
}

impl ThreadPool {
    pub fn new(size: usize) -> Self { todo!() }
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_all_jobs_before_drop_returns() {
        let counter = Arc::new(AtomicUsize::new(0));
        {
            let pool = ThreadPool::new(4);
            for _ in 0..100 {
                let c = Arc::clone(&counter);
                pool.execute(move || { c.fetch_add(1, Ordering::SeqCst); });
            }
        } // drop joins
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
