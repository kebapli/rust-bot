use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// This struct holds global data allocated directly in your RAM.
/// It uses lock-free or highly concurrent types designed for Xeon/Threadripper.
pub struct BotState {
    // A highly parallelized RAM cache that doesn't lock the whole table when accessed
    pub ram_cache: DashMap<String, Vec<u8>>,

    // Lock-free atomic counters for ultra-fast tracking across threads
    pub total_processed_jobs: AtomicU64,
}

impl BotState {
    pub fn new() -> Self {
        Self {
            ram_cache: DashMap::with_capacity(50_000), // Pre-allocates memory for speed
            total_processed_jobs: AtomicU64::new(0),
        }
    }
}
