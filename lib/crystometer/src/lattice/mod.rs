//! Lattice-based measurement organization

use std::collections::HashMap;
use std::time::Duration;

pub struct LatticeMeasurement {
    pub name: String,
    pub samples: Vec<Duration>,
    pub formation_pattern: Vec<f64>,
    pub stability_index: f64,
}

impl LatticeMeasurement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
            formation_pattern: Vec::new(),
            stability_index: 0.93,
        }
    }
}
