//! Crystal Computing Error Types
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 23:35:12 UTC
//! Version: 0.1.0
//! License: MIT

pub mod core;

// Remove the imports since we're defining these types here
// These types should be moved to core.rs instead

// Define MathError with necessary variants
#[derive(Debug)]
pub enum MathError {
    HarmonyStateUnstable,
    InvalidParameter(String),
    DivisionByZero,
    InvalidRange,
    NegativeEnergy,
}

// Define QuantumError as needed
#[derive(Debug)]
pub enum QuantumError {
    IterationLimit,
    ResonanceFailure,
    BoundaryViolation,
    InvalidState,
}

// Define other error types as needed
#[derive(Debug)]
pub enum CrystalError {
    CoherenceLoss,
    AlignmentFailure,
}

#[derive(Debug)]
pub enum VectorError {
    DimensionMismatch,
    NormalizationFailure,
}

#[derive(Debug)]
pub enum CoherenceError {
    PhaseMismatch,
    StabilityLoss,
}

// Define result types for each error type
pub type CrystalResult<T> = Result<T, CrystalError>;
pub type MathResult<T> = Result<T, MathError>;
pub type QuantumResult<T> = Result<T, QuantumError>;
pub type VectorResult<T> = Result<T, VectorError>;
pub type CoherenceResult<T> = Result<T, CoherenceError>;
