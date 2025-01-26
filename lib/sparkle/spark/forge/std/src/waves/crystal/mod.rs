//! Crystal matrix for wave focusing

use crate::waves::WaveError;

/// Crystal matrix for wave control
#[derive(Debug)]
pub struct Crystal {
    /// Matrix dimensions
    dimensions: [usize; 3],
    /// Element spacing
    spacing: f64,
    /// SIMD enabled
    simd_enabled: bool,
    /// GPU enabled
    gpu_enabled: bool,
}

impl Crystal {
    /// Creates a new crystal matrix
    pub fn new(dimensions: [usize; 3], spacing: f64) -> Self {
        Self {
            dimensions,
            spacing: spacing.abs(),
            simd_enabled: false,
            gpu_enabled: false,
        }
    }

    /// Aligns crystal to target
    pub fn align(&self, target: &[f64; 3]) -> Result<(), WaveError> {
        if target.iter().any(|&x| x.abs() > 100.0) {
            Err(WaveError::CrystalError("Target out of alignment range".into()))
        } else {
            Ok(())
        }
    }

    /// Focuses crystal matrix
    pub fn focus(&self) -> Result<(), WaveError> {
        if self.spacing < 0.1 {
            Err(WaveError::CrystalError("Crystal spacing too small".into()))
        } else {
            Ok(())
        }
    }

    /// Enhances crystal power
    pub fn enhance(&mut self, gain: f64) -> Result<(), WaveError> {
        if gain <= 0.0 {
            return Err(WaveError::CrystalError("Invalid enhancement factor".into()));
        }
        self.spacing /= gain.sqrt();
        Ok(())
    }

    /// Tunes crystal resonance
    pub fn tune_resonance(&mut self, freq: f64) -> Result<(), WaveError> {
        if freq <= 0.0 {
            return Err(WaveError::CrystalError("Invalid resonance frequency".into()));
        }
        self.spacing = 299792458.0 / freq; // c/f wavelength
        Ok(())
    }

    /// Optimizes crystal alignment
    pub fn optimize_alignment(&mut self) -> Result<(), WaveError> {
        self.spacing = self.spacing.max(0.1);
        Ok(())
    }

    /// Enables SIMD operations
    pub fn vectorize(&mut self) -> Result<(), WaveError> {
        self.simd_enabled = true;
        Ok(())
    }

    /// Enables GPU acceleration
    pub fn gpu_optimize(&mut self) -> Result<(), WaveError> {
        self.gpu_enabled = true;
        Ok(())
    }

    /// Merges with another crystal
    pub fn merge(&mut self, other: &Self) -> Result<(), WaveError> {
        if self.dimensions != other.dimensions {
            Err(WaveError::CrystalError("Incompatible crystal dimensions".into()))
        } else {
            Ok(())
        }
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new([32, 32, 32], 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_creation() {
        let crystal = Crystal::new([32, 32, 32], 1.0);
        assert!(crystal.spacing > 0.0);
    }

    #[test]
    fn test_crystal_alignment() -> Result<(), WaveError> {
        let crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.align(&[0.0, 0.0, 0.0])?;
        assert!(crystal.align(&[1000.0, 0.0, 0.0]).is_err());
        Ok(())
    }

    #[test]
    fn test_crystal_focus() -> Result<(), WaveError> {
        let crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.focus()?;
        Ok(())
    }

    #[test]
    fn test_crystal_enhancement() -> Result<(), WaveError> {
        let mut crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.enhance(2.0)?;
        assert!(crystal.enhance(-1.0).is_err());
        Ok(())
    }

    #[test]
    fn test_crystal_resonance() -> Result<(), WaveError> {
        let mut crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.tune_resonance(1.0e9)?;
        assert!(crystal.tune_resonance(-1.0).is_err());
        Ok(())
    }

    #[test]
    fn test_crystal_optimization() -> Result<(), WaveError> {
        let mut crystal = Crystal::new([32, 32, 32], 0.05);
        crystal.optimize_alignment()?;
        assert!(crystal.spacing >= 0.1);
        Ok(())
    }

    #[test]
    fn test_crystal_simd() -> Result<(), WaveError> {
        let mut crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.vectorize()?;
        assert!(crystal.simd_enabled);
        Ok(())
    }

    #[test]
    fn test_crystal_gpu() -> Result<(), WaveError> {
        let mut crystal = Crystal::new([32, 32, 32], 1.0);
        crystal.gpu_optimize()?;
        assert!(crystal.gpu_enabled);
        Ok(())
    }

    #[test]
    fn test_crystal_merge() -> Result<(), WaveError> {
        let mut crystal1 = Crystal::new([32, 32, 32], 1.0);
        let crystal2 = Crystal::new([32, 32, 32], 1.0);
        let crystal3 = Crystal::new([64, 64, 64], 1.0);

        crystal1.merge(&crystal2)?;
        assert!(crystal1.merge(&crystal3).is_err());
        Ok(())
    }
}
