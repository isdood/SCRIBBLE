//! Crystal field module for Mark type

/// Crystal field state
#[derive(Debug)]
pub struct Crystal {
    /// Field center
    center: [f64; 3],
    /// Field radius
    radius: f64,
    /// Field strength
    strength: f64,
    /// Field coherence
    coherence: f64,
}

impl Crystal {
    /// Creates a new crystal field
    pub fn new(center: [f64; 3], radius: f64, strength: f64, coherence: f64) -> Self {
        Self {
            center,
            radius: radius.abs(),
            strength: strength.abs(),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    /// Gets the field center
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// Gets the field radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Gets the field strength
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Gets the field coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Resonates the field at a position
    pub fn resonate(&self, position: [f64; 3]) -> Result<(), String> {
        let distance = self.distance_to(position);
        if distance > self.radius {
            Err("Position outside field radius".to_string())
        } else {
            Ok(())
        }
    }

    /// Shifts the field by an offset
    pub fn shift(&self, offset: [f64; 3]) -> Result<(), String> {
        if offset.iter().any(|&x| x.abs() > self.radius) {
            Err("Shift distance exceeds field radius".to_string())
        } else {
            Ok(())
        }
    }

    /// Checks for interference with another field
    pub fn interferes_with(&self, other: &Self) -> Result<bool, String> {
        let distance = self.distance_between_centers(other);
        if distance < self.coherence.min(other.coherence) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Merges with another field
    pub fn merge(&self, other: &Self) -> Result<Self, String> {
        if self.interferes_with(other)? {
            return Err("Cannot merge interfering fields".to_string());
        }

        Ok(Self::new(
            self.average_position(other),
            (self.radius + other.radius) / 2.0,
            (self.strength * other.strength).sqrt(),
            (self.coherence + other.coherence) / 2.0,
        ))
    }

    // Helper methods
    fn distance_to(&self, position: [f64; 3]) -> f64 {
        let squared_dist: f64 = self.center
            .iter()
            .zip(position.iter())
            .map(|(&a, &b)| (b - a).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn distance_between_centers(&self, other: &Self) -> f64 {
        let squared_dist: f64 = self.center
            .iter()
            .zip(other.center.iter())
            .map(|(&a, &b)| (b - a).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn average_position(&self, other: &Self) -> [f64; 3] {
        let mut avg = [0.0; 3];
        for i in 0..3 {
            avg[i] = (self.center[i] + other.center[i]) / 2.0;
        }
        avg
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new([0.0; 3], 1.0, 1.0, 1.0)
    }
}

impl Clone for Crystal {
    fn clone(&self) -> Self {
        Self::new(
            self.center,
            self.radius,
            self.strength,
            self.coherence,
        )
    }
}
