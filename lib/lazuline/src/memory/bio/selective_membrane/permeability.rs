//! Permeability Controller Implementation
//! Created: 2025-01-22
//! Author: isdood

pub struct PermeabilityController {
    permeability_level: f64,
}

impl PermeabilityController {
    pub fn new() -> Self {
        PermeabilityController {
            permeability_level: 1.0,
        }
    }

    pub fn control(&mut self, level: f64) {
        self.permeability_level = level;
    }

    pub fn get_level(&self) -> f64 {
        self.permeability_level
    }
}
