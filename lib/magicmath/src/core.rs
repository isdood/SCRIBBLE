// lib/magicmath/src/core.rs

//! Core Mathematical Operations for Crystal Lattice HPC Systems
//! ===============================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:45:42 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    constants::{
        HARMONY_STABILITY_THRESHOLD,
        RESONANCE_FACTOR
    },
    traits::MeshValue,
    fractal,
    julia,
    brot,
};
use errors::core::MathError;

/// Core harmony state tracking for all mathematical operations
#[derive(Debug, Clone, Copy)]
pub struct HarmonyState {
    pub coherence: f64,
    pub phase: f64,
    pub energy: f64,
    pub stability: f64,
    pub iterations: usize,
}

impl HarmonyState {
    /// Create new harmony state with initial values
    #[inline]
    pub fn new() -> Self {
        Self {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            stability: 1.0,
            iterations: 0,
        }
    }

    /// Check if harmony state is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= HARMONY_STABILITY_THRESHOLD &&
        self.stability >= HARMONY_STABILITY_THRESHOLD
    }

    /// Update harmony state with new operation
    #[inline]
    pub fn update(&mut self, operation: Operation) -> Result<(), MathError> {
        self.coherence *= operation.coherence_factor();
        self.phase += operation.phase_shift();
        self.energy *= operation.energy_factor();
        self.stability *= RESONANCE_FACTOR;
        self.iterations += 1;

        if !self.is_stable() {
            return Err(MathError::HarmonyStateUnstable);
        }
        Ok(())
    }
}

/// Mathematical operations with harmony properties
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    SquareRoot,
    Logarithm,
    Pi,
    Golden,
    Pythagorean,
    Fibonacci,
    Julia,
    Mandelbrot,
    Fractal,
}

impl Operation {
    /// Get coherence factor for operation
    #[inline]
    fn coherence_factor(&self) -> f64 {
        match self {
            Operation::Add => 1.0,
            Operation::Subtract => 0.95,
            Operation::Multiply => 1.05,
            Operation::Divide => 0.90,
            Operation::SquareRoot => 0.85,
            Operation::Logarithm => 0.80,
            Operation::Pi => 1.10,
            Operation::Golden => 1.15,
            Operation::Pythagorean => 0.95,
            Operation::Fibonacci => 1.20,
            Operation::Julia => 1.25,
            Operation::Mandelbrot => 1.30,
            Operation::Fractal => 1.35,
        }
    }

    /// Get phase shift for operation
    #[inline]
    fn phase_shift(&self) -> f64 {
        match self {
            Operation::Add => 0.0,
            Operation::Subtract => core::f64::consts::PI,
            Operation::Multiply => core::f64::consts::PI / 2.0,
            Operation::Divide => -core::f64::consts::PI / 2.0,
            Operation::SquareRoot => core::f64::consts::PI / 4.0,
            Operation::Logarithm => -core::f64::consts::PI / 4.0,
            Operation::Pi => 2.0 * core::f64::consts::PI,
            Operation::Golden => (3.0 - 5.0f64.sqrt()) * core::f64::consts::PI,
            Operation::Pythagorean => core::f64::consts::PI / 3.0,
            Operation::Fibonacci => (1.0 - 5.0f64.sqrt()) * core::f64::consts::PI / 2.0,
            Operation::Julia => core::f64::consts::PI / 6.0,
            Operation::Mandelbrot => core::f64::consts::PI / 8.0,
            Operation::Fractal => core::f64::consts::PI / 12.0,
        }
    }

    /// Get energy factor for operation
    #[inline]
    fn energy_factor(&self) -> f64 {
        match self {
            Operation::Add => 1.1,
            Operation::Subtract => 0.9,
            Operation::Multiply => 1.2,
            Operation::Divide => 0.8,
            Operation::SquareRoot => 0.7,
            Operation::Logarithm => 0.6,
            Operation::Pi => 1.3,
            Operation::Golden => 1.4,
            Operation::Pythagorean => 1.0,
            Operation::Fibonacci => 1.5,
            Operation::Julia => 1.6,
            Operation::Mandelbrot => 1.7,
            Operation::Fractal => 1.8,
        }
    }
}

/// Core mathematical functions with harmony stability
pub struct HarmonyMath {
    state: HarmonyState,
}

impl HarmonyMath {
    /// Create new harmony math instance
    #[inline]
    pub fn new() -> Self {
        Self {
            state: HarmonyState::new(),
        }
    }

    /// Perform operation with harmony stability check
    #[inline]
    pub fn operate<T: MeshValue + Copy>(&mut self, operation: Operation, value: T) -> Result<T, MathError> {
        self.state.update(operation)?;

        match operation {
            Operation::Add => harmony_add(&value, &value),
            Operation::Subtract => harmony_sub(&value, &value),
            Operation::Multiply => harmony_mul(&value, &value),
            Operation::Divide => harmony_div(&value, &value),
            Operation::SquareRoot => harmony_sqrt(&value),
            Operation::Logarithm => harmony_ln(&value),
            Operation::Pi => harmony_pi(&value),
            Operation::Golden => harmony_phi(&value),
            Operation::Pythagorean => harmony_pythagoras(&value, &value),
            Operation::Fibonacci => harmony_fibonacci(value.to_usize()?),
            Operation::Julia => {
                let state = julia::JuliaState::new(value.to_f64()?, 0.0);
                let params = julia::JuliaParams::default();
                let result = julia::iterate_julia(state, &params, julia::JuliaVariant::Standard)?;
                Ok(T::from(result.escape_time().unwrap_or(0) as f64))
            },
            Operation::Mandelbrot => {
                let state = brot::MandelbrotState::new(value.to_f64()?, 0.0);
                let params = brot::MandelbrotParams::default();
                let result = brot::iterate_mandelbrot(state, &params, brot::MandelbrotVariant::Standard)?;
                Ok(T::from(result.escape_time().unwrap_or(0) as f64))
            },
            Operation::Fractal => {
                let state = fractal::FractalState::Custom(fractal::CustomState {
                    z_real: value.to_f64()?,
                                                          z_imag: 0.0,
                                                          iterations: 0,
                                                          stability: 1.0,
                                                          phase: 0.0,
                                                          escape_time: None,
                });
                let params = fractal::FractalParams::default();
                let result = fractal::generate_fractal(state, &params)?;
                Ok(T::from(result.escape_time().unwrap_or(0) as f64))
            },
        }
    }

    /// Get current harmony state
    #[inline]
    pub fn get_state(&self) -> HarmonyState {
        self.state
    }

    /// Reset harmony state
    #[inline]
    pub fn reset_state(&mut self) {
        self.state = HarmonyState::new();
    }
}

// Core harmony operations
#[inline]
pub fn harmony_add<T: MeshValue>(a: &T, b: &T) -> Result<T, MathError> {
    Ok(T::from(a.to_f64()? + b.to_f64()?))
}

#[inline]
pub fn harmony_sub<T: MeshValue>(a: &T, b: &T) -> Result<T, MathError> {
    Ok(T::from(a.to_f64()? - b.to_f64()?))
}

#[inline]
pub fn harmony_mul<T: MeshValue>(a: &T, b: &T) -> Result<T, MathError> {
    Ok(T::from(a.to_f64()? * b.to_f64()?))
}

#[inline]
pub fn harmony_div<T: MeshValue>(a: &T, b: &T) -> Result<T, MathError> {
    let b_val = b.to_f64()?;
    if b_val == 0.0 {
        return Err(MathError::DivisionByZero);
    }
    Ok(T::from(a.to_f64()? / b_val))
}

#[inline]
pub fn harmony_sqrt<T: MeshValue>(a: &T) -> Result<T, MathError> {
    let val = a.to_f64()?;
    if val < 0.0 {
        return Err(MathError::InvalidDomain("Square root of negative number".to_string()));
    }
    Ok(T::from(val.sqrt()))
}

#[inline]
pub fn harmony_ln<T: MeshValue>(a: &T) -> Result<T, MathError> {
    let val = a.to_f64()?;
    if val <= 0.0 {
        return Err(MathError::LogarithmDomainError(val));
    }
    Ok(T::from(val.ln()))
}

#[inline]
pub fn harmony_pi<T: MeshValue>(scale: &T) -> Result<T, MathError> {
    Ok(T::from(core::f64::consts::PI * scale.to_f64()?))
}

#[inline]
pub fn harmony_phi<T: MeshValue>(scale: &T) -> Result<T, MathError> {
    Ok(T::from(((1.0 + 5.0f64.sqrt()) / 2.0) * scale.to_f64()?))
}

#[inline]
pub fn harmony_pythagoras<T: MeshValue>(a: &T, b: &T) -> Result<T, MathError> {
    let a_val = a.to_f64()?;
    let b_val = b.to_f64()?;
    Ok(T::from((a_val * a_val + b_val * b_val).sqrt()))
}

#[inline]
pub fn harmony_fibonacci<T: MeshValue>(n: usize) -> Result<T, MathError> {
    if n <= 1 {
        return Ok(T::from(n as f64));
    }
    let mut a = 0.0;
    let mut b = 1.0;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    Ok(T::from(b))
}
