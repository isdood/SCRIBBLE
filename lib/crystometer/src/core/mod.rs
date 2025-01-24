//! Core functionality for crystal-based benchmarking

use std::time::{Duration, Instant};

pub struct CrystalTimer {
    start_time: Option<Instant>,
    total_duration: Duration,
    formation_count: u64,
}

impl CrystalTimer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            total_duration: Duration::new(0, 0),
            formation_count: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.total_duration += start.elapsed();
            self.formation_count += 1;
        }
    }
}
