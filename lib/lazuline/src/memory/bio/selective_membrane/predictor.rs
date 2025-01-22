//! Access Predictor Implementation
//! Created: 2025-01-22
//! Author: isdood

pub struct AccessPredictor {
    access_pattern: Vec<String>,
}

impl AccessPredictor {
    pub fn new() -> Self {
        AccessPredictor {
            access_pattern: Vec::new(),
        }
    }

    pub fn record_access(&mut self, key: String) {
        self.access_pattern.push(key);
    }

    pub fn predict(&self) -> Option<&String> {
        // Simple prediction: return the most recently accessed key
        self.access_pattern.last()
    }
}
