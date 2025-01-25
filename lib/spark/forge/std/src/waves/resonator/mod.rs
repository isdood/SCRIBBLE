//! Resonator system for wave transmission

use crate::waves::WaveError;

/// Resonator state for wave transmission
#[derive(Debug)]
pub struct Resonator {
    /// Base frequency
    frequency: f64,
    /// Power level
    power: f64,
    /// SIMD enabled
    simd_enabled: bool,
    /// GPU enabled
    gpu_enabled: bool,
    /// Parallel mode
    parallel_mode: bool,
}

impl Resonator {
    /// Creates a new resonator
    pub fn new(frequency: f64, power: f64) -> Self {
        Self {
            frequency: frequency.abs(),
            power: power.abs(),
            simd_enabled: false,
            gpu_enabled: false,
            parallel_mode: false,
        }
    }

    /// Activates the resonator
    pub fn activate(&self) -> Result<(), WaveError> {
        if self.power < 1.0 {
            Err(WaveError::ResonanceError("Insufficient power for activation".into()))
        } else {
            Ok(())
        }
    }

    /// Transmits data through resonator
    pub fn transmit(&self, data: &[u8]) -> Result<usize, WaveError> {
        if !self.is_stable() {
            return Err(WaveError::ResonanceError("Resonator unstable".into()));
        }
        Ok(data.len())
    }

    /// Receives data through resonator
    pub fn receive(&self, buffer: &mut [u8]) -> Result<usize, WaveError> {
        if !self.is_stable() {
            return Err(WaveError::ResonanceError("Resonator unstable".into()));
        }
        Ok(buffer.len())
    }

    /// Tunes to a source
    pub fn tune(&self, source: &[f64; 3]) -> Result<(), WaveError> {
        if self.distance_to(source) > 100.0 {
            Err(WaveError::ResonanceError("Source out of range".into()))
        } else {
            Ok(())
        }
    }

    /// Amplifies resonator power
    pub fn amplify(&mut self, gain: f64) -> Result<(), WaveError> {
        if gain <= 0.0 {
            return Err(WaveError::ResonanceError("Invalid gain factor".into()));
        }
        self.power *= gain;
        Ok(())
    }

    /// Tunes resonator frequency
    pub fn tune_frequency(&mut self, freq: f64) -> Result<(), WaveError> {
        if freq <= 0.0 {
            return Err(WaveError::ResonanceError("Invalid frequency".into()));
        }
        self.frequency = freq;
        Ok(())
    }

    /// Enables SIMD operations
    pub fn enable_simd(&mut self) -> Result<(), WaveError> {
        self.simd_enabled = true;
        Ok(())
    }

    /// Enables GPU acceleration
    pub fn gpu_accelerate(&mut self) -> Result<(), WaveError> {
        self.gpu_enabled = true;
        Ok(())
    }

    /// Enables parallel transmission
    pub fn enable_parallel(&mut self) -> Result<(), WaveError> {
        self.parallel_mode = true;
        Ok(())
    }

    /// Synchronizes with another resonator
    pub fn synchronize(&mut self, other: &Self) -> Result<(), WaveError> {
        if (self.frequency - other.frequency).abs() > 1.0e6 {
            Err(WaveError::ResonanceError("Frequency mismatch too large".into()))
        } else {
            Ok(())
        }
    }

    // Helper methods
    fn is_stable(&self) -> bool {
        self.power >= 1.0 && self.frequency > 0.0
    }

    fn distance_to(&self, point: &[f64; 3]) -> f64 {
        point.iter().map(|&x| x.powi(2)).sum::<f64>().sqrt()
    }
}

impl Default for Resonator {
    fn default() -> Self {
        Self::new(1.0e9, 1.0)
    }
}
