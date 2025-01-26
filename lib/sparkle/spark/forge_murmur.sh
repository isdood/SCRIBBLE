#!/bin/bash

# Spark Murmur Module Setup Script
# Author: isdood
# Created: 2025-01-25 20:15:47 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized 16-bit integer type

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_murmur_module() {
    cd forge/std || exit 1

    # 1. Create murmur module structure
    mkdir -p src/murmur/{wave,crystal}
    mkdir -p tests/murmur

    # 2. Update lib.rs with murmur module
    if ! grep -q "pub mod murmur;" src/lib.rs; then
        sed -i '/pub mod whisper;/a pub mod murmur;' src/lib.rs
        sed -i '/pub use whisper::WhisperResult;/a pub use murmur::{Murmur, MurmurResult};' src/lib.rs
    fi

    # 3. Create main murmur module file
    cat > src/murmur/mod.rs << 'EOL'
//! Crystal-optimized 16-bit integer type.
//!
//! This module provides a high-performance 16-bit integer type optimized for
//! crystal-space wave propagation and harmonic resonance.

pub mod wave;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use wave::WaveState;
use crystal::CrystalResonance;

/// Result type for murmur operations
pub type MurmurResult<T> = Result<T, MurmurError>;

/// Error type for murmur operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MurmurError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Wave resonance error
    WaveError(String),
}

/// Crystal-optimized 16-bit integer
#[derive(Debug, Clone, Copy)]
pub struct Murmur {
    /// The raw 16-bit value
    value: i16,
    /// Wave propagation state
    wave: WaveState,
    /// Crystal resonance state
    resonance: CrystalResonance,
}

impl PartialEq for Murmur {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value &&
        self.wave.approx_eq(&other.wave) &&
        self.resonance.approx_eq(&other.resonance)
    }
}

impl Eq for Murmur {}

impl Murmur {
    /// Creates a new Murmur value
    pub fn new(value: i16) -> Self {
        Self {
            value,
            wave: WaveState::default(),
            resonance: CrystalResonance::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Gets the wave state
    pub fn wave(&self) -> &WaveState {
        &self.wave
    }

    /// Gets the crystal resonance
    pub fn resonance(&self) -> &CrystalResonance {
        &self.resonance
    }

    /// Performs a checked addition with wave propagation
    pub fn checked_add(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(MurmurError::Overflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.propagate(&rhs.wave),
                resonance: self.resonance.combine(&rhs.resonance),
            })
        }
    }

    /// Performs a checked subtraction with wave interference
    pub fn checked_sub(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(MurmurError::Underflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.interfere(&rhs.wave),
                resonance: self.resonance.combine(&rhs.resonance),
            })
        }
    }

    /// Performs a checked multiplication with resonance amplification
    pub fn checked_mul(self, rhs: Self) -> MurmurResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(MurmurError::Overflow)
        } else {
            Ok(Self {
                value,
                wave: self.wave.amplify(&rhs.wave),
                resonance: self.resonance.amplify(&rhs.resonance),
            })
        }
    }

    /// Performs a checked division with wave attenuation
    pub fn checked_div(self, rhs: Self) -> MurmurResult<Self> {
        if rhs.value == 0 {
            Err(MurmurError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(MurmurError::Overflow)
            } else {
                Ok(Self {
                    value,
                    wave: self.wave.attenuate(&rhs.wave),
                    resonance: self.resonance.attenuate(&rhs.resonance),
                })
            }
        }
    }
}

impl Default for Murmur {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i16> for Murmur {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl Add for Murmur {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Murmur {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Murmur {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Murmur {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Murmur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for Murmur {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Murmur {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
EOL

    # 4. Create wave module
    cat > src/murmur/wave/mod.rs << 'EOL'
//! Wave propagation module for Murmur type

use std::f64::consts::PI;

/// Wave propagation state
#[derive(Debug, Clone, Copy)]
pub struct WaveState {
    amplitude: f64,
    frequency: f64,
    phase: f64,
}

impl WaveState {
    /// Creates a new wave state
    pub fn new(amplitude: f64, frequency: f64, phase: f64) -> Self {
        Self {
            amplitude: amplitude.abs(),
            frequency: frequency.abs(),
            phase: phase % (2.0 * PI),
        }
    }

    /// Gets the wave amplitude
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// Gets the wave frequency
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Gets the wave phase
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// Propagates with another wave
    pub fn propagate(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude + other.amplitude,
            (self.frequency + other.frequency) / 2.0,
            self.phase + other.phase,
        )
    }

    /// Interferes with another wave
    pub fn interfere(&self, other: &Self) -> Self {
        Self::new(
            (self.amplitude.powi(2) + other.amplitude.powi(2)).sqrt(),
            (self.frequency * other.frequency).sqrt(),
            (self.phase - other.phase).abs(),
        )
    }

    /// Amplifies with another wave
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude * other.amplitude,
            self.frequency * other.frequency,
            self.phase * other.phase,
        )
    }

    /// Attenuates with another wave
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.amplitude / other.amplitude.max(1.0),
            self.frequency / other.frequency.max(1.0),
            self.phase / other.phase.max(1.0),
        )
    }

    /// Check if wave states are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.amplitude - other.amplitude).abs() < f64::EPSILON &&
        (self.frequency - other.frequency).abs() < f64::EPSILON &&
        (self.phase - other.phase).abs() < f64::EPSILON
    }
}

impl Default for WaveState {
    fn default() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }
}
EOL

    # 5. Create crystal module
    cat > src/murmur/crystal/mod.rs << 'EOL'
//! Crystal resonance module for Murmur type

/// Crystal resonance state
#[derive(Debug, Clone, Copy)]
pub struct CrystalResonance {
    energy: f64,
    coherence: f64,
}

impl CrystalResonance {
    /// Creates a new crystal resonance
    pub fn new(energy: f64, coherence: f64) -> Self {
        Self {
            energy: energy.abs(),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    /// Gets the resonance energy
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Gets the resonance coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Combines with another resonance
    pub fn combine(&self, other: &Self) -> Self {
        Self::new(
            (self.energy + other.energy) / 2.0,
            (self.coherence * other.coherence).sqrt(),
        )
    }

    /// Amplifies with another resonance
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.energy * other.energy,
            self.coherence * other.coherence,
        )
    }

    /// Attenuates with another resonance
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.energy / other.energy.max(1.0),
            self.coherence / other.coherence.max(1.0),
        )
    }

    /// Check if resonances are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.energy - other.energy).abs() < f64::EPSILON &&
        (self.coherence - other.coherence).abs() < f64::EPSILON
    }
}

impl Default for CrystalResonance {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}
EOL

    # 6. Create test module
    cat > tests/murmur/mod.rs << 'EOL'
use spark_std::murmur::{Murmur, MurmurResult};

#[test]
fn test_murmur_creation() {
    let m = Murmur::new(42);
    assert_eq!(m.get(), 42);
}

#[test]
fn test_murmur_arithmetic() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    assert_eq!((a + b).get(), 3000);
    assert_eq!((b - a).get(), 1000);
    assert_eq!((a * b).get(), 2_000_000);
    assert_eq!((b / a).get(), 2);
}

#[test]
fn test_murmur_wave_propagation() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    let c = a.checked_add(b).unwrap();
    assert_eq!(c.get(), 3000);
    assert!(c.wave().amplitude() > 0.0);
    assert!(c.wave().frequency() > 0.0);
}

#[test]
fn test_murmur_resonance() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);

    let c = a.checked_mul(b).unwrap();
    assert_eq!(c.get(), 2_000_000);
    assert!(c.resonance().energy() > 0.0);
    assert!(c.resonance().coherence() > 0.0);
}

#[test]
fn test_murmur_comparison() {
    let a = Murmur::new(1000);
    let b = Murmur::new(2000);
    let c = Murmur::new(1000);

    assert!(a < b);
    assert!(a <= c);
    assert!(b > a);
    assert!(a >= c);
    assert_eq!(a, c);
    assert_ne!(a, b);
}
EOL

    print_purple "✓ Created murmur module files"
}

main() {
    print_purple "🔮 Creating Spark Murmur Module..."
    setup_murmur_module
    print_purple "✨ Murmur module created with crystal-wave optimization!

Features:
- Crystal-optimized 16-bit integers
- Wave propagation mechanics
- Crystal resonance system
- Safe arithmetic operations
- Wave interference patterns
- Resonance amplification
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
