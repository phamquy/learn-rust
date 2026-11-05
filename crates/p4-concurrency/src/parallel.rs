//! Split work across threads with std::thread::scope, then again with Arc<Mutex<_>>, then with mpsc.
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Sum `data` using `n_threads` scoped threads (no Arc needed).
pub fn parallel_sum_scoped(data: &[u64], n_threads: usize) -> u64 {
    todo!()
}

/// Same result, but accumulate into an Arc<Mutex<u64>> using 'static threads (data is moved in via Arc).
pub fn parallel_sum_mutex(data: Arc<Vec<u64>>, n_threads: usize) -> u64 {
    todo!()
}

/// Same again, but each worker sends its partial sum over an mpsc channel.
pub fn parallel_sum_channel(data: Arc<Vec<u64>>, n_threads: usize) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<u64> { (1..=1000).collect() }
    const EXPECTED: u64 = 500_500;

    #[test]
    fn scoped() { assert_eq!(parallel_sum_scoped(&data(), 4), EXPECTED); }

    #[test]
    fn mutex() { assert_eq!(parallel_sum_mutex(Arc::new(data()), 4), EXPECTED); }

    #[test]
    fn channel() { assert_eq!(parallel_sum_channel(Arc::new(data()), 4), EXPECTED); }

    #[test]
    fn more_threads_than_items() { assert_eq!(parallel_sum_scoped(&[1, 2], 8), 3); }
}
