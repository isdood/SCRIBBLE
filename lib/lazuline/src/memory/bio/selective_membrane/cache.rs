//! Membrane Cache Implementation
//! Created: 2025-01-22
//! Author: isdood

use std::collections::HashMap;

pub struct MembraneCache {
    cache: HashMap<String, String>,
    capacity: usize,
}

impl MembraneCache {
    pub fn new(capacity: usize) -> Self {
        MembraneCache {
            cache: HashMap::new(),
            capacity,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }

    pub fn put(&mut self, key: String, value: String) {
        if self.cache.len() >= self.capacity {
            // Simple eviction policy: remove a random item
            let first_key = self.cache.keys().next().cloned().unwrap();
            self.cache.remove(&first_key);
        }
        self.cache.insert(key, value);
    }

    pub fn optimize(&mut self) {
        // TODO: Implement cache optimization logic
    }
}
