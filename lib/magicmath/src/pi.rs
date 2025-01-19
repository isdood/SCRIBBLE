// lib/magicmath/src/pi.rs

//! PI Operations for Crystal Lattice HPC Systems
//! =========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 10:19:07 UTC
//! Version: 0.1.0
//! License: MIT

use crate::{
    errors::MathError,
    constants::{
        MAX_LATTICE_SIZE,
        MIN_LATTICE_SIZE,
        HARMONY_STABILITY_THRESHOLD,
        RESONANCE_FACTOR,
        PHASE_CIRCULAR_FACTOR,
        HARMONY_CIRCULARITY_THRESHOLD,
        CONVERGENCE_THRESHOLD,
        PI,
        TAU
    },
    traits::MeshValue,
};

/// Harmony-aware PI scaling for crystal lattice values
/// Handles circular energy distribution and phase alignment
pub fn harmony_pi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let coherence = calculate_circularity(x)?;
    if coherence < HARMONY_STABILITY_THRESHOLD {
        return Err(MathError::CoherenceLoss);
    }

    let result = stabilized_pi(x)?;
    validate_lattice_bounds(result)?;

    Ok(result)
}

/// Calculate PI-based values with harmony circularity preservation
pub fn circular_pi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let circularity = check_circularity(x)?;
    if !circularity.is_stable() {
        return Err(MathError::CircularityLoss);
    }

    harmony_pi(x)
}

/// Calculate PI approximation using harmony-adapted Leibniz series
pub fn leibniz_pi<T: MeshValue>(iterations: usize) -> Result<T, MathError> {
    let mut result = T::zero();
    let mut sign = T::unit();
    let mut denom = T::unit();

    for _ in 0..iterations {
        let term = sign.raw_div(denom)?;
        result = result.raw_add(term)?;
        sign = sign.raw_mul(T::from(-1.0))?;
        denom = denom.raw_add(T::from(2.0))?;
    }

    Ok(result.raw_mul(T::from(4.0))?)
}

/// Ramanujan's harmony-adapted PI series
pub fn ramanujan_pi<T: MeshValue>(iterations: usize) -> Result<T, MathError> {
    let mut result = T::zero();
    let mut k = T::zero();

    for _ in 0..iterations {
        let numerator = factorial(4 * k.to_usize()?)? * (1103 + 26390 * k.to_usize()?);
        let denominator = pow_factorial(4 * k.to_usize()?)? * pow(396, 4 * k.to_usize()?)?;

        let term = T::from(numerator as f64).raw_div(T::from(denominator as f64))?;
        result = result.raw_add(term)?;
        k = k.raw_add(T::unit())?;
    }

    let factor = T::from(2.0 * 2.0f64.sqrt() / 9801.0);
    Ok(T::unit().raw_div(result.raw_mul(factor)?)?)
}

/// Calculate circular functions with phase preservation
pub fn phase_circle<T: MeshValue>(x: T, phase: f64) -> Result<(T, T), MathError> {
    if !is_valid_phase(phase) {
        return Err(MathError::PhaseError);
    }

    let phase_aligned = apply_phase_alignment(x, phase)?;
    Ok((harmony_sin(phase_aligned)?, harmony_cos(phase_aligned)?))
}

// Internal helper functions

#[inline]
fn calculate_circularity<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let base_coherence = x.coherence()? * PI;
    Ok(base_coherence * RESONANCE_FACTOR)
}

#[inline]
fn stabilized_pi<T: MeshValue>(x: T) -> Result<T, MathError> {
    let pi_scaled = x.raw_mul(T::from(PI))?;
    let circular_correction = calculate_circular_correction(x)?;
    apply_circular_correction(pi_scaled, circular_correction)
}

#[inline]
fn calculate_circular_correction<T: MeshValue>(x: T) -> Result<f64, MathError> {
    let energy_ratio = x.energy()? / PI;
    Ok((energy_ratio * RESONANCE_FACTOR).sin())
}

#[inline]
fn apply_circular_correction<T: MeshValue>(value: T, correction: f64) -> Result<T, MathError> {
    value.circularize(correction)
}

#[inline]
fn validate_lattice_bounds<T: MeshValue>(value: T) -> Result<(), MathError> {
    let mag = value.magnitude()?;
    if mag < MIN_LATTICE_SIZE as f64 || mag > MAX_LATTICE_SIZE as f64 {
        return Err(MathError::LatticeBoundsError);
    }
    Ok(())
}

#[inline]
fn check_circularity<T: MeshValue>(x: T) -> Result<CircularityState, MathError> {
    let circularity = x.circularity_state()?;
    Ok(circularity)
}

#[inline]
fn is_valid_phase(phase: f64) -> bool {
    phase >= 0.0 && phase <= TAU
}

#[inline]
fn apply_phase_alignment<T: MeshValue>(x: T, phase: f64) -> Result<T, MathError> {
    let aligned_phase = phase * PHASE_CIRCULAR_FACTOR;
    x.phase_align(aligned_phase)
}

#[inline]
fn harmony_sin<T: MeshValue>(x: T) -> Result<T, MathError> {
    let mut result = x;
    let mut term = x;
    let mut n = T::from(1.0);

    while term.magnitude()? > CONVERGENCE_THRESHOLD {
        n = n.raw_add(T::from(2.0))?;
        term = term.raw_mul(x.raw_mul(x)?)?
        .raw_mul(T::from(-1.0))?
        .raw_div(T::from(n * (n - T::unit())?))?;
        result = result.raw_add(term)?;
    }

    Ok(result)
}

#[inline]
fn harmony_cos<T: MeshValue>(x: T) -> Result<T, MathError> {
    let mut result = T::unit();
    let mut term = T::unit();
    let mut n = T::zero();

    while term.magnitude()? > CONVERGENCE_THRESHOLD {
        n = n.raw_add(T::from(2.0))?;
        term = term.raw_mul(x.raw_mul(x)?)?
        .raw_mul(T::from(-1.0))?
        .raw_div(T::from(n * (n - T::unit())?))?;
        result = result.raw_add(term)?;
    }

    Ok(result)
}

#[inline]
fn factorial(n: usize) -> Result<usize, MathError> {
    if n == 0 {
        return Ok(1);
    }
    Ok((1..=n).product())
}

#[inline]
fn pow_factorial(n: usize) -> Result<usize, MathError> {
    Ok((1..=n).map(|_| factorial(n).unwrap()).product())
}

#[inline]
fn pow(base: usize, exp: usize) -> Result<usize, MathError> {
    Ok((0..exp).fold(1, |acc, _| acc * base))
}

/// Harmony circularity state for PI calculations
#[derive(Debug, Clone, Copy)]
pub struct CircularityState {
    coherence: f64,
    phase: f64,
    energy: f64,
    circularity: f64,
}

impl CircularityState {
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.coherence >= HARMONY_STABILITY_THRESHOLD &&
        self.circularity >= HARMONY_CIRCULARITY_THRESHOLD
    }

    #[inline]
    pub fn rotate(&self, angle: f64) -> Self {
        Self {
            coherence: self.coherence,
            phase: (self.phase + angle) % TAU,
            energy: self.energy,
            circularity: self.circularity * RESONANCE_FACTOR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test implementation of MeshValue for f64
    impl MeshValue for f64 {
        fn coherence(&self) -> Result<f64, MathError> { Ok(1.0) }
        fn energy(&self) -> Result<f64, MathError> { Ok(*self) }
        fn magnitude(&self) -> Result<f64, MathError> { Ok(self.abs()) }
        fn raw_add(&self, other: Self) -> Result<Self, MathError> { Ok(self + other) }
        fn raw_mul(&self, other: Self) -> Result<Self, MathError> { Ok(self * other) }
        fn raw_div(&self, other: Self) -> Result<Self, MathError> { Ok(self / other) }
        fn from(value: f64) -> Self { value }
        fn zero() -> Self { 0.0 }
        fn unit() -> Self { 1.0 }
        fn to_usize(&self) -> Result<usize, MathError> { Ok(*self as usize) }
        fn circularize(&self, correction: f64) -> Result<Self, MathError> {
            Ok(self * (1.0 + correction * (PI - 3.0)))
        }
        fn phase_align(&self, phase: f64) -> Result<Self, MathError> {
            Ok(self * (phase * PHASE_CIRCULAR_FACTOR).cos())
        }
        fn circularity_state(&self) -> Result<CircularityState, MathError> {
            Ok(CircularityState {
                coherence: 1.0,
                phase: 0.0,
                energy: *self,
                circularity: 1.0,
            })
        }
    }

    #[test]
    fn test_harmony_pi() {
        let result = harmony_pi(1.0).unwrap();
        assert!((result - PI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_circular_pi() {
        let result = circular_pi(1.0).unwrap();
        assert!((result - PI).abs() < CONVERGENCE_THRESHOLD);
    }

    #[test]
    fn test_leibniz_pi() {
        let result = leibniz_pi::<f64>(1000).unwrap();
        assert!((result - PI).abs() < 0.01);
    }

    #[test]
    fn test_ramanujan_pi() {
        let result = ramanujan_pi::<f64>(2).unwrap();
        assert!((result - PI).abs() < 1e-10);
    }

    #[test]
    fn test_phase_circle() {
        let (sin, cos) = phase_circle(PI/2.0, 0.0).unwrap();
        assert!((sin - 1.0).abs() < CONVERGENCE_THRESHOLD);
        assert!(cos.abs() < CONVERGENCE_THRESHOLD);

        assert!(phase_circle(PI/2.0, -1.0).is_err()); // Invalid phase
    }

    #[test]
    fn test_circularity_state() {
        let state = CircularityState {
            coherence: 1.0,
            phase: 0.0,
            energy: 1.0,
            circularity: 1.0,
        };
        assert!(state.is_stable());

        let rotated = state.rotate(PI);
        assert_eq!(rotated.phase, PI);
    }
}
