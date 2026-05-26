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

    /// Computes bucket index for given key
    pub fn calc_bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.buckets.len() as u64) as usize
    }

    pub fn load_factor(&self) -> f64 {
        if self.buckets.is_empty() {
            0.0 as f64
        } else {
            self.len as f64 / self.buckets.len() as f64
        }
    }

    /// Insert or update a key - value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.load_factor() > 0.75 {
            self.resize();
        }

        let index = self.calc_bucket_index(&key);
        let bucket = &mut self.buckets[index];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value)); //update existing and return
            }
        }
        bucket.push((key, value));
        self.len += 1;
        None
    }

    /// Search for a key (will return reference to value)
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.calc_bucket_index(key);
        let bucket = &self.buckets[index];
        for (k, v) in bucket.iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}
