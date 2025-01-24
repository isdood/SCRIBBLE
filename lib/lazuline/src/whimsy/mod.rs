//! Whimsy engine implementation module

/// Engine for handling whimsical computations
pub struct WhimsyEngine {
    level: u8,
}

impl WhimsyEngine {
    /// Creates a new WhimsyEngine with the specified level
    pub fn new(level: u8) -> Self {
        Self {
            level,
        }
    }

    /// Gets the current whimsy level
    pub fn get_level(&self) -> u8 {
        self.level
    }

    /// Sets a new whimsy level
    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }
}
