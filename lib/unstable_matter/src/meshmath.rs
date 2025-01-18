/// Ethereal Vector Crystallography Module
/// Last Updated: 2025-01-18 18:44:30 UTC
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
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self::crystallize(x, y, z)
    }

    /// Create a new vector without bounds checking
    #[inline(always)]
    pub const fn new_unchecked(x: T, y: T, z: T) -> Self {
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

    /// Get x-component (compatibility method)
    #[inline(always)]
    pub fn get_x(&self) -> T { self.prime }

    /// Access y-component (compatibility method)
    #[inline(always)]
    pub fn y(&self) -> T { self.resonant }

    /// Get y-component (compatibility method)
    #[inline(always)]
    pub fn get_y(&self) -> T { self.resonant }

    /// Access z-component (compatibility method)
    #[inline(always)]
    pub fn z(&self) -> T { self.harmonic }

    /// Get z-component (compatibility method)
    #[inline(always)]
    pub fn get_z(&self) -> T { self.harmonic }

    /// Access prime axis component
    #[inline(always)]
    pub fn prime(&self) -> T { self.prime }

    /// Access resonant axis component
    #[inline(always)]
    pub fn resonant(&self) -> T { self.resonant }

    /// Access harmonic axis component
    #[inline(always)]
    pub fn harmonic(&self) -> T { self.harmonic }

    /// Get mutable reference to vector
    #[inline(always)]
    pub fn get(&self) -> &Self {
        self
    }

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

// ... rest of implementation remains the same ...

impl<T: MeshValue + Copy + Into<f64>> Vector3D<T> {
    /// Calculate absolute resonance magnitude
    #[inline]
    pub fn mesh_magnitude(&self) -> f64 {
        let p: f64 = self.prime.into();
        let r: f64 = self.resonant.into();
        let h: f64 = self.harmonic.into();
        MeshMath::sqrt_f64(p * p + r * r + h * h)
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

// Additional operator implementations for references
impl<'a, 'b, T: MeshValue + Copy> std::ops::Add<&'b Vector3D<T>> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn add(self, other: &'b Vector3D<T>) -> Self::Output {
        self.mesh_add(other)
    }
}

impl<'a, 'b, T: MeshValue + Copy> std::ops::Sub<&'b Vector3D<T>> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn sub(self, other: &'b Vector3D<T>) -> Self::Output {
        self.mesh_sub(other)
    }
}

impl<'a, T: MeshValue + Copy> std::ops::Mul<T> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn mul(self, resonance: T) -> Self::Output {
        self.mesh_mul(resonance)
    }
}

impl<'a, T: MeshValue + Copy> std::ops::Div<T> for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn div(self, resonance: T) -> Self::Output {
        self.mesh_div(resonance)
    }
}

impl<'a, T: MeshValue + Copy> std::ops::Neg for &'a Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.mesh_neg()
    }
}

// Add reference implementations for right-hand operands
impl<T: MeshValue + Copy> std::ops::Add<Vector3D<T>> for &Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn add(self, other: Vector3D<T>) -> Self::Output {
        self.mesh_add(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Sub<Vector3D<T>> for &Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn sub(self, other: Vector3D<T>) -> Self::Output {
        self.mesh_sub(&other)
    }
}

impl<T: MeshValue + Copy> std::ops::Add<&Vector3D<T>> for Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn add(self, other: &Vector3D<T>) -> Self::Output {
        self.mesh_add(other)
    }
}

impl<T: MeshValue + Copy> std::ops::Sub<&Vector3D<T>> for Vector3D<T> {
    type Output = Vector3D<T>;

    #[inline]
    fn sub(self, other: &Vector3D<T>) -> Self::Output {
        self.mesh_sub(other)
    }
}

// AsRef implementation for convenient borrowing
impl<T: MeshValue + Copy> AsRef<Vector3D<T>> for Vector3D<T> {
    #[inline]
    fn as_ref(&self) -> &Vector3D<T> {
        self
    }
}

// From implementations for common conversions
impl<T: MeshValue + Copy> From<(T, T, T)> for Vector3D<T> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl<T: MeshValue + Copy> From<[T; 3]> for Vector3D<T> {
    #[inline]
    fn from(array: [T; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

// Iterator methods
impl<T: MeshValue + Copy> IntoIterator for Vector3D<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.prime, self.resonant, self.harmonic].into_iter()
    }
}

impl<'a, T: MeshValue + Copy> IntoIterator for &'a Vector3D<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        [self.prime, self.resonant, self.harmonic].into_iter()
    }
}
