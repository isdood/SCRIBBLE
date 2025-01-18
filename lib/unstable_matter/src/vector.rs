/// Ethereal Vector Crystallography Module
/// Last Updated: 2025-01-18 18:31:47 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    scribe::{Scribe, ScribePrecision, QuantumString},
    quantum::Quantum,
    meshmath::{MeshMath, MeshValue},
};

/// Three-dimensional ethereal vector structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T> where T: MeshValue + Copy {
    /// Prismatic axis (x-component)
    prime: T,
    /// Resonant axis (y-component)
    resonant: T,
    /// Harmonic axis (z-component)
    harmonic: T,
}

impl<T: MeshValue + Copy> Vector3D<T> {
    /// Create a new vector (compatibility method)
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self::crystallize(x, y, z)
    }

    /// Crystallize a new vector from raw components
    #[inline(always)]
    pub const fn crystallize(prime: T, resonant: T, harmonic: T) -> Self {
        Self { prime, resonant, harmonic }
    }

    /// Create a null-space vector
    #[inline(always)]
    pub fn void() -> Self {
        Self::crystallize(T::mesh_zero(), T::mesh_zero(), T::mesh_zero())
    }

    /// Create a unity-resonance vector
    #[inline(always)]
    pub fn unity() -> Self {
        Self::crystallize(T::mesh_one(), T::mesh_one(), T::mesh_one())
    }

    /// Access x-component (compatibility method)
    #[inline(always)]
    pub fn x(&self) -> T { self.prime }

    /// Access y-component (compatibility method)
    #[inline(always)]
    pub fn y(&self) -> T { self.resonant }

    /// Access z-component (compatibility method)
    #[inline(always)]
    pub fn z(&self) -> T { self.harmonic }

    /// Access prime axis component
    #[inline(always)]
    pub fn prime(&self) -> T { self.prime }

    /// Access resonant axis component
    #[inline(always)]
    pub fn resonant(&self) -> T { self.resonant }

    /// Access harmonic axis component
    #[inline(always)]
    pub fn harmonic(&self) -> T { self.harmonic }

    /// Normalize vector (compatibility method)
    #[inline]
    pub fn normalize(&self) -> Self where T: Into<f64> + From<f64> {
        self.mesh_normalize()
    }

    /// Calculate magnitude (compatibility method)
    #[inline]
    pub fn magnitude(&self) -> f64 where T: Into<f64> {
        self.mesh_magnitude()
    }

    /// Harmonically combine vectors
    #[inline]
    pub fn mesh_add(&self, other: &Self) -> Self {
        Self::crystallize(
            self.prime.mesh_add(other.prime),
                          self.resonant.mesh_add(other.resonant),
                          self.harmonic.mesh_add(other.harmonic)
        )
    }

    /// Extract ethereal difference
    #[inline]
    pub fn mesh_sub(&self, other: &Self) -> Self {
        Self::crystallize(
            self.prime.mesh_sub(other.prime),
                          self.resonant.mesh_sub(other.resonant),
                          self.harmonic.mesh_sub(other.harmonic)
        )
    }

    /// Amplify vector by resonance factor
    #[inline]
    pub fn mesh_mul(&self, resonance: T) -> Self {
        Self::crystallize(
            self.prime.mesh_mul(resonance),
                          self.resonant.mesh_mul(resonance),
                          self.harmonic.mesh_mul(resonance)
        )
    }

    /// Attenuate vector by resonance factor
    #[inline]
    pub fn mesh_div(&self, resonance: T) -> Self {
        Self::crystallize(
            self.prime.mesh_div(resonance),
                          self.resonant.mesh_div(resonance),
                          self.harmonic.mesh_div(resonance)
        )
    }

    /// Invert vector polarity
    #[inline]
    pub fn mesh_neg(&self) -> Self {
        Self::crystallize(
            self.prime.mesh_neg(),
                          self.resonant.mesh_neg(),
                          self.harmonic.mesh_neg()
        )
    }

    /// Calculate harmonic resonance between vectors
    #[inline]
    pub fn mesh_dot(&self, other: &Self) -> T {
        self.prime.mesh_mul(other.prime)
        .mesh_add(self.resonant.mesh_mul(other.resonant))
        .mesh_add(self.harmonic.mesh_mul(other.harmonic))
    }

    /// Measure total resonance magnitude
    #[inline]
    pub fn mesh_magnitude_squared(&self) -> T {
        self.mesh_dot(self)
    }

    /// Generate crystalline cross-resonance
    #[inline]
    pub fn mesh_cross(&self, other: &Self) -> Self {
        Self::crystallize(
            self.resonant.mesh_mul(other.harmonic).mesh_sub(self.harmonic.mesh_mul(other.resonant)),
                          self.harmonic.mesh_mul(other.prime).mesh_sub(self.prime.mesh_mul(other.harmonic)),
                          self.prime.mesh_mul(other.resonant).mesh_sub(self.resonant.mesh_mul(other.prime))
        )
    }
}

impl<T: MeshValue + Copy + Into<f64>> Vector3D<T> {
    /// Calculate absolute resonance magnitude
    #[inline]
    pub fn mesh_magnitude(&self) -> f64 {
        let p: f64 = self.prime.into();
        let r: f64 = self.resonant.into();
        let h: f64 = self.harmonic.into();
        (p * p + r * r + h * h).sqrt()
    }

    /// Normalize to unit resonance
    #[inline]
    pub fn mesh_normalize(&self) -> Self where T: From<f64> {
        let magnitude = self.mesh_magnitude();
        if MeshMath::eq_f64(magnitude, 0.0) {
            *self
        } else {
            let factor: T = T::from(1.0 / magnitude);
            self.mesh_mul(factor)
        }
    }

    /// Calculate ethereal distance
    #[inline]
    pub fn mesh_distance(&self, other: &Self) -> f64 {
        self.mesh_sub(other).mesh_magnitude()
    }
}

impl Vector3D<isize> {
    /// Calculate quantum-aligned distance
    #[inline]
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let diff = self.mesh_sub(other);
        let dp = MeshMath::isize_to_f64(diff.prime);
        let dr = MeshMath::isize_to_f64(diff.resonant);
        let dh = MeshMath::isize_to_f64(diff.harmonic);

        MeshMath::sqrt_f64(
            dp.mesh_mul(dp)
            .mesh_add(dr.mesh_mul(dr))
            .mesh_add(dh.mesh_mul(dh))
        )
    }
}

impl<T: MeshValue + Scribe + Copy> Scribe for Vector3D<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨∴");  // Ethereal vector symbol
        self.prime.scribe(precision, output);
        output.push_str("∥");   // Parallel symbol
        self.resonant.scribe(precision, output);
        output.push_str("∥");
        self.harmonic.scribe(precision, output);
        output.push_str("∴⟩");
    }
}

impl<T: MeshValue + Copy> Default for Vector3D<T> {
    fn default() -> Self {
        Self::void()
    }
}

impl<T: MeshValue + Copy> Quantum for Vector3D<T> {
    #[inline(always)]
    fn get_coherence(&self) -> f64 {
        1.0  // Perfect crystalline coherence
    }

    #[inline(always)]
    fn is_quantum_stable(&self) -> bool {
        true  // Vectors maintain eternal stability
    }

    #[inline(always)]
    fn decay_coherence(&self) {}  // Vectors resist entropy

    #[inline(always)]
    fn reset_coherence(&self) {}  // No reset needed for perfect forms
}

// Standard operators
impl<T: MeshValue + Copy> std::ops::Add for Vector3D<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.mesh_add(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Sub for Vector3D<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.mesh_sub(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Mul<T> for Vector3D<T> {
    type Output = Self;

    #[inline]
    fn mul(self, resonance: T) -> Self::Output {
        self.mesh_mul(resonance)
    }
}

impl<T: MeshValue + Copy> std::ops::Div<T> for Vector3D<T> {
    type Output = Self;

    #[inline]
    fn div(self, resonance: T) -> Self::Output {
        self.mesh_div(resonance)
    }
}

impl<T: MeshValue + Copy> std::ops::Neg for Vector3D<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.mesh_neg()
    }
}

/// Hypercrystalline Vector Implementation - Fourth Dimensional Resonance
/// Last Updated: 2025-01-18 18:22:46 UTC
/// Author: isdood
/// Current User: isdood

/// Four-dimensional ethereal vector structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4D<T> where T: MeshValue + Copy {
    /// Prismatic axis
    pub prime: T,
    /// Resonant axis
    pub resonant: T,
    /// Harmonic axis
    pub harmonic: T,
    /// Ethereal axis (transcendent dimension)
    pub aetheric: T,
}

impl<T: MeshValue + Copy> Vector4D<T> {
    /// Crystallize a hyperdimensional vector
    #[inline(always)]
    pub const fn crystallize(prime: T, resonant: T, harmonic: T, aetheric: T) -> Self {
        Self {
            prime,
            resonant,
            harmonic,
            aetheric
        }
    }

    /// Create a hyperspace null vector
    #[inline(always)]
    pub fn void() -> Self {
        Self::crystallize(
            T::mesh_zero(),
                          T::mesh_zero(),
                          T::mesh_zero(),
                          T::mesh_zero()
        )
    }

    /// Create a hyperspace unity vector
    #[inline(always)]
    pub fn unity() -> Self {
        Self::crystallize(
            T::mesh_one(),
                          T::mesh_one(),
                          T::mesh_one(),
                          T::mesh_one()
        )
    }

    /// Access prismatic component
    #[inline(always)]
    pub fn prime(&self) -> T { self.prime }

    /// Access resonant component
    #[inline(always)]
    pub fn resonant(&self) -> T { self.resonant }

    /// Access harmonic component
    #[inline(always)]
    pub fn harmonic(&self) -> T { self.harmonic }

    /// Access aetheric component
    #[inline(always)]
    pub fn aetheric(&self) -> T { self.aetheric }

    /// Harmonically combine hypervectors
    #[inline]
    pub fn mesh_add(&self, other: &Self) -> Self {
        Self::crystallize(
            self.prime.mesh_add(other.prime),
                          self.resonant.mesh_add(other.resonant),
                          self.harmonic.mesh_add(other.harmonic),
                          self.aetheric.mesh_add(other.aetheric)
        )
    }

    /// Extract hyperspace difference
    #[inline]
    pub fn mesh_sub(&self, other: &Self) -> Self {
        Self::crystallize(
            self.prime.mesh_sub(other.prime),
                          self.resonant.mesh_sub(other.resonant),
                          self.harmonic.mesh_sub(other.harmonic),
                          self.aetheric.mesh_sub(other.aetheric)
        )
    }

    /// Amplify hypervector by resonance factor
    #[inline]
    pub fn mesh_mul(&self, resonance: T) -> Self {
        Self::crystallize(
            self.prime.mesh_mul(resonance),
                          self.resonant.mesh_mul(resonance),
                          self.harmonic.mesh_mul(resonance),
                          self.aetheric.mesh_mul(resonance)
        )
    }

    /// Attenuate hypervector by resonance factor
    #[inline]
    pub fn mesh_div(&self, resonance: T) -> Self {
        Self::crystallize(
            self.prime.mesh_div(resonance),
                          self.resonant.mesh_div(resonance),
                          self.harmonic.mesh_div(resonance),
                          self.aetheric.mesh_div(resonance)
        )
    }

    /// Invert hyperspace polarity
    #[inline]
    pub fn mesh_neg(&self) -> Self {
        Self::crystallize(
            self.prime.mesh_neg(),
                          self.resonant.mesh_neg(),
                          self.harmonic.mesh_neg(),
                          self.aetheric.mesh_neg()
        )
    }

    /// Calculate hyperdimensional resonance
    #[inline]
    pub fn mesh_dot(&self, other: &Self) -> T {
        self.prime.mesh_mul(other.prime)
        .mesh_add(self.resonant.mesh_mul(other.resonant))
        .mesh_add(self.harmonic.mesh_mul(other.harmonic))
        .mesh_add(self.aetheric.mesh_mul(other.aetheric))
    }

    /// Measure total hyperresonance magnitude
    #[inline]
    pub fn mesh_magnitude_squared(&self) -> T {
        self.mesh_dot(self)
    }
}

impl Vector4D<f64> {
    /// Calculate absolute hyperresonance magnitude
    #[inline]
    pub fn mesh_magnitude(&self) -> f64 {
        MeshMath::sqrt_f64(self.mesh_magnitude_squared())
    }

    /// Normalize to unit hyperresonance
    #[inline]
    pub fn mesh_normalize(&self) -> Self {
        let magnitude = self.mesh_magnitude();
        if MeshMath::eq_f64(magnitude, 0.0) {
            *self
        } else {
            self.mesh_div(magnitude)
        }
    }

    /// Calculate ethereal hyperdistance
    #[inline]
    pub fn mesh_distance(&self, other: &Self) -> f64 {
        self.mesh_sub(other).mesh_magnitude()
    }
}

impl Vector4D<isize> {
    /// Calculate quantum-aligned hyperdistance
    #[inline]
    pub fn quantum_distance(&self, other: &Self) -> f64 {
        let diff = self.mesh_sub(other);
        let dp = MeshMath::isize_to_f64(diff.prime);
        let dr = MeshMath::isize_to_f64(diff.resonant);
        let dh = MeshMath::isize_to_f64(diff.harmonic);
        let da = MeshMath::isize_to_f64(diff.aetheric);

        MeshMath::sqrt_f64(
            dp.mesh_mul(dp)
            .mesh_add(dr.mesh_mul(dr))
            .mesh_add(dh.mesh_mul(dh))
            .mesh_add(da.mesh_mul(da))
        )
    }
}

impl<T: MeshValue + Scribe + Copy> Scribe for Vector4D<T> {
    fn scribe(&self, precision: ScribePrecision, output: &mut QuantumString) {
        output.push_str("⟨∷");  // Hyperspace vector symbol
        self.prime.scribe(precision, output);
        output.push_str("∥");    // Parallel symbol
        self.resonant.scribe(precision, output);
        output.push_str("∥");
        self.harmonic.scribe(precision, output);
        output.push_str("∥");
        self.aetheric.scribe(precision, output);
        output.push_str("∷⟩");
    }
}

impl<T: MeshValue + Copy> Quantum for Vector4D<T> {
    #[inline(always)]
    fn get_coherence(&self) -> f64 {
        1.0  // Perfect hypercrystalline coherence
    }

    #[inline(always)]
    fn is_quantum_stable(&self) -> bool {
        true  // Hypervectors maintain eternal stability
    }

    #[inline(always)]
    fn decay_coherence(&self) {}  // Hypervectors transcend entropy

    #[inline(always)]
    fn reset_coherence(&self) {}  // No reset needed for perfect hyperforms
}

// Hypercrystalline operator implementations
impl<T: MeshValue + Copy> std::ops::Add for Vector4D<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.mesh_add(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Sub for Vector4D<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.mesh_sub(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Mul<T> for Vector4D<T> {
    type Output = Self;

    #[inline]
    fn mul(self, resonance: T) -> Self::Output {
        self.mesh_mul(resonance)
    }
}

impl<T: MeshValue + Copy> std::ops::Div<T> for Vector4D<T> {
    type Output = Self;

    #[inline]
    fn div(self, resonance: T) -> Self::Output {
        self.mesh_div(resonance)
    }
}

impl<T: MeshValue + Copy> std::ops::Neg for Vector4D<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.mesh_neg()
    }
}
