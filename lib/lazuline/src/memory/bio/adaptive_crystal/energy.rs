//! Energy Tracker Implementation
//! Created: 2025-01-22
//! Author: isdood

pub struct EnergyTracker {
    energy_consumed: f64,
}

impl EnergyTracker {
    pub fn new() -> Self {
        EnergyTracker {
            energy_consumed: 0.0,
        }
    }

    pub fn track(&mut self, energy: f64) {
        self.energy_consumed += energy;
    }

    pub fn get_energy(&self) -> f64 {
        self.energy_consumed
    }
}
