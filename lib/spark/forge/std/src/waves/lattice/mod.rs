//! Lattice grid for wave propagation

use crate::waves::WaveError;

/// Lattice grid for wave control
#[derive(Debug)]
pub struct Lattice {
    /// Grid size
    size: [usize; 3],
    /// Node spacing
    spacing: f64,
    /// Grid efficiency
    efficiency: f64,
    /// SIMD enabled
    simd_enabled: bool,
    /// GPU enabled
    gpu_enabled: bool,
}

impl Lattice {
    /// Creates a new lattice grid
    pub fn new(size: [usize; 3], spacing: f64) -> Self {
        Self {
            size,
            spacing: spacing.abs(),
            efficiency: 1.0,
            simd_enabled: false,
            gpu_enabled: false,
        }
    }

    /// Configures lattice for data size
    pub fn configure(&self, data_size: usize) -> Result<(), WaveError> {
        let capacity = self.size.iter().product::<usize>();
        if data_size > capacity {
            Err(WaveError::LatticeError("Data size exceeds lattice capacity".into()))
        } else {
            Ok(())
        }
    }

    /// Prepares lattice for reception
    pub fn prepare(&self) -> Result<(), WaveError> {
        if self.efficiency < 0.5 {
            Err(WaveError::LatticeError("Lattice efficiency too low".into()))
        } else {
            Ok(())
        }
    }

    /// Strengthens lattice nodes
    pub fn strengthen(&mut self, gain: f64) -> Result<(), WaveError> {
        if gain <= 0.0 {
            return Err(WaveError::LatticeError("Invalid strength factor".into()));
        }
        self.efficiency *= gain.min(2.0);
        Ok(())
    }

    /// Adjusts node spacing
    pub fn adjust_spacing(&mut self, frequency: f64) -> Result<(), WaveError> {
        if frequency <= 0.0 {
            return Err(WaveError::LatticeError("Invalid frequency".into()));
        }
        self.spacing = (299792458.0 / frequency) / 32.0; // wavelength/32
        Ok(())
    }

    /// Enables SIMD operations
    pub fn enable_vectorization(&mut self) -> Result<(), WaveError> {
        self.simd_enabled = true;
        self.efficiency *= 1.5;
        Ok(())
    }

    /// Enables parallel processing
    pub fn parallelize(&mut self) -> Result<(), WaveError> {
        if self.size.iter().any(|&x| x < 16) {
            Err(WaveError::LatticeError("Grid too small for parallelization".into()))
        } else {
            self.efficiency *= 1.2;
            Ok(())
        }
    }

    /// Enables GPU computation
    pub fn gpu_compute(&mut self) -> Result<(), WaveError> {
        self.gpu_enabled = true;
        self.efficiency *= 2.0;
        Ok(())
    }

    /// Combines with another lattice
    pub fn combine(&mut self, other: &Self) -> Result<(), WaveError> {
        if self.spacing != other.spacing {
            Err(WaveError::LatticeError("Incompatible lattice spacing".into()))
        } else {
            Ok(())
        }
    }
}

impl Default for Lattice {
    fn default() -> Self {
        Self::new([64, 64, 64], 1.0)
    }
}
