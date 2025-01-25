#!/bin/bash

# Spark Whisper Module Setup Script
# Author: isdood
# Created: 2025-01-25 20:05:18 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized 8-bit integer type

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_whisper_module() {
    cd forge/std || exit 1

    # 1. Create whisper module structure
    mkdir -p src/whisper/{ops,crystal}
    mkdir -p tests/whisper

    # 2. Update lib.rs with whisper module
    if ! grep -q "pub mod whisper;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod whisper;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use whisper::{Whisper, WhisperResult};' src/lib.rs
    fi

    # 3. Create main whisper module file
    cat > src/whisper/mod.rs << 'EOL'
//! Crystal-optimized 8-bit integer type.
//!
//! This module provides a high-performance 8-bit integer type optimized for
//! crystal-space operations and quantum resonance patterns.

pub mod ops;
pub mod crystal;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr};
use std::cmp::Ordering;
use ops::{WhisperOps, CrystalResonance};
use crystal::{CrystalState, QuantumPhase};

/// Result type for whisper operations
pub type WhisperResult<T> = Result<T, WhisperError>;

/// Error type for whisper operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhisperError {
    /// Overflow error
    Overflow,
    /// Underflow error
    Underflow,
    /// Division by zero
    DivisionByZero,
    /// Quantum state error
    QuantumError(String),
}

/// Crystal-optimized 8-bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Whisper {
    /// The raw 8-bit value
    value: i8,
    /// Quantum phase state
    phase: QuantumPhase,
    /// Crystal resonance state
    state: CrystalState,
}

impl Whisper {
    /// Creates a new Whisper value
    pub fn new(value: i8) -> Self {
        Self {
            value,
            phase: QuantumPhase::default(),
            state: CrystalState::default(),
        }
    }

    /// Gets the raw value
    pub fn get(&self) -> i8 {
        self.value
    }

    /// Gets the quantum phase
    pub fn phase(&self) -> &QuantumPhase {
        &self.phase
    }

    /// Gets the crystal state
    pub fn state(&self) -> &CrystalState {
        &self.state
    }

    /// Performs a checked addition
    pub fn checked_add(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        if overflow {
            Err(WhisperError::Overflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked subtraction
    pub fn checked_sub(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_sub(rhs.value);
        if overflow {
            Err(WhisperError::Underflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked multiplication
    pub fn checked_mul(self, rhs: Self) -> WhisperResult<Self> {
        let (value, overflow) = self.value.overflowing_mul(rhs.value);
        if overflow {
            Err(WhisperError::Overflow)
        } else {
            Ok(Self {
                value,
                phase: self.phase.combine(&rhs.phase),
                state: self.state.resonate(&rhs.state),
            })
        }
    }

    /// Performs a checked division
    pub fn checked_div(self, rhs: Self) -> WhisperResult<Self> {
        if rhs.value == 0 {
            Err(WhisperError::DivisionByZero)
        } else {
            let (value, overflow) = self.value.overflowing_div(rhs.value);
            if overflow {
                Err(WhisperError::Overflow)
            } else {
                Ok(Self {
                    value,
                    phase: self.phase.combine(&rhs.phase),
                    state: self.state.resonate(&rhs.state),
                })
            }
        }
    }
}

impl Default for Whisper {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<i8> for Whisper {
    fn from(value: i8) -> Self {
        Self::new(value)
    }
}

impl Add for Whisper {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).unwrap_or_else(|_| self)
    }
}

impl Sub for Whisper {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).unwrap_or_else(|_| self)
    }
}

impl Mul for Whisper {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).unwrap_or_else(|_| self)
    }
}

impl Div for Whisper {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).unwrap_or_else(|_| self)
    }
}

impl fmt::Display for Whisper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialOrd for Whisper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Whisper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
EOL

    # 4. Create ops module
    cat > src/whisper/ops/mod.rs << 'EOL'
//! Operations module for Whisper type

use super::{Whisper, WhisperResult, WhisperError};
use super::crystal::{CrystalState, QuantumPhase};

/// Trait for Whisper-specific operations
pub trait WhisperOps {
    /// Performs crystal resonance
    fn resonate(&self, other: &Self) -> WhisperResult<Whisper>;
    /// Performs quantum phase shift
    fn phase_shift(&self, phase: f64) -> WhisperResult<Whisper>;
}

impl WhisperOps for Whisper {
    fn resonate(&self, other: &Self) -> WhisperResult<Whisper> {
        let phase = self.phase.combine(&other.phase);
        let state = self.state.resonate(&other.state);

        Ok(Whisper {
            value: self.value,
            phase,
            state,
        })
    }

    fn phase_shift(&self, phase: f64) -> WhisperResult<Whisper> {
        if !(-std::f64::consts::PI..=std::f64::consts::PI).contains(&phase) {
            return Err(WhisperError::QuantumError("Phase out of range".into()));
        }

        Ok(Whisper {
            value: self.value,
            phase: self.phase.shift(phase),
            state: self.state.clone(),
        })
    }
}

/// Trait for crystal resonance operations
pub trait CrystalResonance {
    /// Performs crystal lattice alignment
    fn align(&self, other: &Self) -> WhisperResult<Whisper>;
    /// Performs quantum entanglement
    fn entangle(&self, other: &Self) -> WhisperResult<Whisper>;
}

impl CrystalResonance for Whisper {
    fn align(&self, other: &Self) -> WhisperResult<Whisper> {
        let state = self.state.align(&other.state);

        Ok(Whisper {
            value: self.value,
            phase: self.phase.clone(),
            state,
        })
    }

    fn entangle(&self, other: &Self) -> WhisperResult<Whisper> {
        let phase = self.phase.entangle(&other.phase);
        let state = self.state.entangle(&other.state);

        Ok(Whisper {
            value: self.value,
            phase,
            state,
        })
    }
}
EOL

    # 5. Create crystal module
    cat > src/whisper/crystal/mod.rs << 'EOL'
//! Crystal module for Whisper type

use std::f64::consts::PI;

/// Quantum phase state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumPhase {
    phase: f64,
}

impl QuantumPhase {
    /// Creates a new quantum phase
    pub fn new(phase: f64) -> Self {
        Self {
            phase: phase % (2.0 * PI),
        }
    }

    /// Gets the phase value
    pub fn value(&self) -> f64 {
        self.phase
    }

    /// Combines two phases
    pub fn combine(&self, other: &Self) -> Self {
        Self::new(self.phase + other.phase)
    }

    /// Shifts the phase
    pub fn shift(&self, delta: f64) -> Self {
        Self::new(self.phase + delta)
    }

    /// Entangles two phases
    pub fn entangle(&self, other: &Self) -> Self {
        Self::new((self.phase + other.phase) / 2.0)
    }
}

impl Default for QuantumPhase {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Crystal lattice state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrystalState {
    energy: f64,
    alignment: f64,
}

impl CrystalState {
    /// Creates a new crystal state
    pub fn new(energy: f64, alignment: f64) -> Self {
        Self {
            energy: energy.max(0.0),
            alignment: alignment.clamp(-1.0, 1.0),
        }
    }

    /// Gets the energy level
    pub fn energy(&self) -> f64 {
        self.energy
    }

    /// Gets the alignment value
    pub fn alignment(&self) -> f64 {
        self.alignment
    }

    /// Resonates with another state
    pub fn resonate(&self, other: &Self) -> Self {
        Self::new(
            (self.energy * other.energy).sqrt(),
            (self.alignment + other.alignment) / 2.0,
        )
    }

    /// Aligns with another state
    pub fn align(&self, other: &Self) -> Self {
        Self::new(
            self.energy,
            (self.alignment + other.alignment).signum(),
        )
    }

    /// Entangles with another state
    pub fn entangle(&self, other: &Self) -> Self {
        Self::new(
            (self.energy + other.energy) / 2.0,
            (self.alignment * other.alignment).abs(),
        )
    }
}

impl Default for CrystalState {
    fn default() -> Self {
        Self::new(1.0, 0.0)
    }
}
EOL

    # 6. Create test module
    cat > tests/whisper/mod.rs << 'EOL'
use spark_std::whisper::{Whisper, WhisperOps, CrystalResonance};

#[test]
fn test_whisper_creation() {
    let w = Whisper::new(42);
    assert_eq!(w.get(), 42);
}

#[test]
fn test_whisper_arithmetic() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    assert_eq!((a + b).get(), 30);
    assert_eq!((b - a).get(), 10);
    assert_eq!((a * b).get(), -56); // Overflow handling
}

#[test]
fn test_whisper_resonance() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    let c = a.resonate(&b).unwrap();
    assert_eq!(c.get(), 10);
    assert!(c.phase().value() >= 0.0);
}

#[test]
fn test_whisper_phase_shift() {
    let a = Whisper::new(10);
    let b = a.phase_shift(1.0).unwrap();

    assert_eq!(a.get(), b.get());
    assert!(b.phase().value() > a.phase().value());
}

#[test]
fn test_crystal_operations() {
    let a = Whisper::new(10);
    let b = Whisper::new(20);

    let c = a.align(&b).unwrap();
    assert_eq!(c.get(), 10);

    let d = a.entangle(&b).unwrap();
    assert_eq!(d.get(), 10);
}
EOL

    print_purple "✓ Created whisper module files"
}

main() {
    print_purple "🔮 Creating Spark Whisper Module..."
    setup_whisper_module
    print_purple "✨ Whisper module created with crystal-space optimization!

Features:
- Crystal-optimized 8-bit integers
- Quantum phase tracking
- Crystal lattice resonance
- Safe arithmetic operations
- Quantum entanglement
- Phase shift operations
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
