//! This is a custom hasmap implementation that uses side chaining
//! using vectors

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Initializes HashMap with given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let buckets = (0..capacity).map(|_| Vec::new()).collect();
        Self { buckets, len: 0 }
    }
}
