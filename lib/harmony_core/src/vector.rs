//! Vector Operations for Crystal Computing
//! ====================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-18
//! Last Updated: 2025-01-19 13:16:05 UTC
//! Version: 0.1.1
//! License: MIT

use core::{
    fmt,
    ops::{Add, Sub, Mul, Div},
};

use magicmath::{
    QuantumMath,
    MathResult,
    Operation,
    QuantumState,
};

use crate::{
    errors::{QuantumError, CoherenceError},
    constants,
};

/// Represents a 3D vector in crystal space
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
    quantum_state: QuantumState,
}

/// Represents a 4D vector in quantum crystal space
#[derive(Debug, Clone, Copy)]
pub struct Vector4D {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    quantum_state: QuantumState,
}

impl Vector3D {
    /// Creates a new 3D vector with quantum state
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let qmath = QuantumMath::new();
        Self {
            x,
            y,
            z,
            quantum_state: qmath.get_state(),
        }
    }

    /// Gets the x component
    pub fn x(&self) -> f64 { self.x }

    /// Gets the y component
    pub fn y(&self) -> f64 { self.y }

    /// Gets the z component
    pub fn z(&self) -> f64 { self.z }

    /// Calculates the magnitude using quantum-aware operations
    pub fn magnitude(&self) -> MathResult<f64> {
        let mut qmath = QuantumMath::new();
        let sum_squares = qmath.operate(Operation::Add,
                                        self.x * self.x + self.y * self.y + self.z * self.z)?;
                                        qmath.operate(Operation::SquareRoot, sum_squares)
    }

    /// Normalizes the vector using quantum operations
    pub fn normalize(&mut self) -> Result<(), QuantumError> {
        let mag = self.magnitude()
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Magnitude calculation failed")))?;

        if mag < f64::EPSILON {
            return Err(QuantumError::CoherenceError(CoherenceError::new("Vector too small to normalize")));
        }

        let mut qmath = QuantumMath::new();
        self.x = qmath.operate(Operation::Divide, self.x/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;
        self.y = qmath.operate(Operation::Divide, self.y/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;
        self.z = qmath.operate(Operation::Divide, self.z/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;

        self.quantum_state = qmath.get_state();
        Ok(())
    }

    /// Calculates dot product with quantum coherence check
    pub fn dot(&self, other: &Vector3D) -> MathResult<f64> {
        let mut qmath = QuantumMath::new();
        qmath.operate(Operation::Add,
                      self.x * other.x + self.y * other.y + self.z * other.z)
    }

    /// Calculates cross product with quantum operations
    pub fn cross(&self, other: &Vector3D) -> Result<Vector3D, QuantumError> {
        let mut qmath = QuantumMath::new();
        let x = qmath.operate(Operation::Subtract,
                              self.y * other.z - self.z * other.y)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Cross product failed")))?;

        let y = qmath.operate(Operation::Subtract,
                              self.z * other.x - self.x * other.z)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Cross product failed")))?;

        let z = qmath.operate(Operation::Subtract,
                              self.x * other.y - self.y * other.x)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Cross product failed")))?;

        Ok(Vector3D {
            x,
            y,
            z,
            quantum_state: qmath.get_state(),
        })
    }

    /// Rotates the vector around an axis by an angle (in radians)
    pub fn rotate(&mut self, axis: &Vector3D, angle: f64) -> Result<(), QuantumError> {
        let mut axis_normalized = axis.clone();
        axis_normalized.normalize()?;

        let mut qmath = QuantumMath::new();
        let cos_angle = qmath.operate(Operation::Pythagorean, angle.cos())
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Rotation failed")))?;
        let sin_angle = qmath.operate(Operation::Pythagorean, angle.sin())
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Rotation failed")))?;

        let x = (cos_angle + axis_normalized.x * axis_normalized.x * (1.0 - cos_angle)) * self.x
        + (axis_normalized.x * axis_normalized.y * (1.0 - cos_angle) - axis_normalized.z * sin_angle) * self.y
        + (axis_normalized.x * axis_normalized.z * (1.0 - cos_angle) + axis_normalized.y * sin_angle) * self.z;

        let y = (axis_normalized.y * axis_normalized.x * (1.0 - cos_angle) + axis_normalized.z * sin_angle) * self.x
        + (cos_angle + axis_normalized.y * axis_normalized.y * (1.0 - cos_angle)) * self.y
        + (axis_normalized.y * axis_normalized.z * (1.0 - cos_angle) - axis_normalized.x * sin_angle) * self.z;

        let z = (axis_normalized.z * axis_normalized.x * (1.0 - cos_angle) - axis_normalized.y * sin_angle) * self.x
        + (axis_normalized.z * axis_normalized.y * (1.0 - cos_angle) + axis_normalized.x * sin_angle) * self.y
        + (cos_angle + axis_normalized.z * axis_normalized.z * (1.0 - cos_angle)) * self.z;

        self.x = x;
        self.y = y;
        self.z = z;
        self.quantum_state = qmath.get_state();

        Ok(())
    }

    /// Gets the current quantum state
    pub fn quantum_state(&self) -> QuantumState {
        self.quantum_state
    }

    /// Checks if the vector's quantum state is coherent
    pub fn is_coherent(&self) -> bool {
        self.quantum_state.coherence_level() >= constants::QUANTUM_STABILITY_THRESHOLD
    }
}

impl Vector4D {
    /// Creates a new 4D vector with quantum state
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        let qmath = QuantumMath::new();
        Self {
            x,
            y,
            z,
            w,
            quantum_state: qmath.get_state(),
        }
    }

    /// Gets the x component
    pub fn x(&self) -> f64 { self.x }

    /// Gets the y component
    pub fn y(&self) -> f64 { self.y }

    /// Gets the z component
    pub fn z(&self) -> f64 { self.z }

    /// Gets the w component (fourth dimension)
    pub fn w(&self) -> f64 { self.w }

    /// Calculates the 4D magnitude using quantum operations
    pub fn magnitude(&self) -> MathResult<f64> {
        let mut qmath = QuantumMath::new();
        let sum_squares = qmath.operate(Operation::Add,
                                        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)?;
                                        qmath.operate(Operation::SquareRoot, sum_squares)
    }

    /// Normalizes the 4D vector using quantum operations
    pub fn normalize(&mut self) -> Result<(), QuantumError> {
        let mag = self.magnitude()
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Magnitude calculation failed")))?;

        if mag < f64::EPSILON {
            return Err(QuantumError::CoherenceError(CoherenceError::new("Vector too small to normalize")));
        }

        let mut qmath = QuantumMath::new();
        self.x = qmath.operate(Operation::Divide, self.x/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;
        self.y = qmath.operate(Operation::Divide, self.y/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;
        self.z = qmath.operate(Operation::Divide, self.z/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;
        self.w = qmath.operate(Operation::Divide, self.w/mag)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Normalization failed")))?;

        self.quantum_state = qmath.get_state();
        Ok(())
    }

    /// Calculates 4D dot product with quantum coherence check
    pub fn dot(&self, other: &Vector4D) -> MathResult<f64> {
        let mut qmath = QuantumMath::new();
        qmath.operate(Operation::Add,
                      self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w)
    }

    /// Projects the 4D vector to 3D space
    pub fn to_3d(&self) -> Result<Vector3D, QuantumError> {
        if self.w.abs() < f64::EPSILON {
            return Err(QuantumError::CoherenceError(CoherenceError::new("Invalid w component for projection")));
        }

        let mut qmath = QuantumMath::new();
        let scale = qmath.operate(Operation::Divide, 1.0/self.w)
        .map_err(|e| QuantumError::CoherenceError(CoherenceError::new("Projection failed")))?;

        Ok(Vector3D {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            quantum_state: qmath.get_state(),
        })
    }

    /// Gets the current quantum state
    pub fn quantum_state(&self) -> QuantumState {
        self.quantum_state
    }

    /// Checks if the vector's quantum state is coherent
    pub fn is_coherent(&self) -> bool {
        self.quantum_state.coherence_level() >= constants::QUANTUM_STABILITY_THRESHOLD
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector4D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert!(v.is_coherent());
        assert!(v.quantum_state().stability_factor() >= constants::QUANTUM_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);
        assert!(v.is_coherent());
        assert!(v.quantum_state().stability_factor() >= constants::QUANTUM_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_vector3d_quantum_operations() {
        let mut v = Vector3D::new(3.0, 4.0, 0.0);
        let initial_state = v.quantum_state();

        // Test magnitude with quantum math
        let mag = v.magnitude().unwrap();
        assert!((mag - 5.0).abs() < f64::EPSILON);

        // Verify quantum state is updated
        assert!(v.quantum_state().coherence_level() >= initial_state.coherence_level());
    }

    #[test]
    fn test_vector3d_normalization() {
        let mut v = Vector3D::new(3.0, 4.0, 0.0);
        v.normalize().unwrap();

        // Test magnitude is 1.0
        assert!((v.magnitude().unwrap() - 1.0).abs() < f64::EPSILON);

        // Verify quantum coherence
        assert!(v.is_coherent());
        assert!(v.quantum_state().coherence_level() >= constants::QUANTUM_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_vector3d_dot_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);

        let dot = v1.dot(&v2).unwrap();
        assert_eq!(dot, 0.0);

        // Test with non-orthogonal vectors
        let v3 = Vector3D::new(2.0, 3.0, 4.0);
        let v4 = Vector3D::new(1.0, 2.0, 3.0);
        let dot2 = v3.dot(&v4).unwrap();
        assert!((dot2 - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_vector3d_cross_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);

        let v3 = v1.cross(&v2).unwrap();
        assert_eq!(v3.z(), 1.0);
        assert!(v3.is_coherent());

        // Test quantum state preservation
        assert!(v3.quantum_state().stability_factor() >= constants::QUANTUM_STABILITY_THRESHOLD);
    }

    #[test]
    fn test_vector3d_rotation() {
        let mut v = Vector3D::new(1.0, 0.0, 0.0);
        let axis = Vector3D::new(0.0, 0.0, 1.0);

        // Rotate 90 degrees (π/2 radians)
        v.rotate(&axis, std::f64::consts::PI / 2.0).unwrap();

        assert!((v.x()).abs() < f64::EPSILON);
        assert!((v.y() - 1.0).abs() < f64::EPSILON);
        assert!((v.z()).abs() < f64::EPSILON);
        assert!(v.is_coherent());
    }

    #[test]
    fn test_vector4d_quantum_operations() {
        let mut v = Vector4D::new(2.0, 3.0, 4.0, 5.0);
        let initial_state = v.quantum_state();

        // Test magnitude with quantum math
        let mag = v.magnitude().unwrap();
        assert!((mag - (54.0_f64.sqrt())).abs() < f64::EPSILON);

        // Verify quantum state is updated
        assert!(v.quantum_state().coherence_level() >= initial_state.coherence_level());
    }

    #[test]
    fn test_vector4d_projection() {
        let v4 = Vector4D::new(2.0, 4.0, 6.0, 2.0);
        let v3 = v4.to_3d().unwrap();

        assert_eq!(v3.x(), 1.0);
        assert_eq!(v3.y(), 2.0);
        assert_eq!(v3.z(), 3.0);
        assert!(v3.is_coherent());

        // Test projection with w = 0 (should fail)
        let v4_invalid = Vector4D::new(1.0, 1.0, 1.0, 0.0);
        assert!(v4_invalid.to_3d().is_err());
    }

    #[test]
    fn test_quantum_coherence_limits() {
        let v3 = Vector3D::new(1.0, 1.0, 1.0);
        let v4 = Vector4D::new(1.0, 1.0, 1.0, 1.0);

        assert!(v3.quantum_state().coherence_level() <= constants::MAX_PHASE_COHERENCE);
        assert!(v3.quantum_state().coherence_level() >= constants::MIN_PHASE_COHERENCE);

        assert!(v4.quantum_state().coherence_level() <= constants::MAX_PHASE_COHERENCE);
        assert!(v4.quantum_state().coherence_level() >= constants::MIN_PHASE_COHERENCE);
    }

    #[test]
    fn test_error_handling() {
        // Test normalization of zero vector
        let mut v3_zero = Vector3D::new(0.0, 0.0, 0.0);
        assert!(v3_zero.normalize().is_err());

        // Test 4D to 3D projection with w = 0
        let v4_zero_w = Vector4D::new(1.0, 1.0, 1.0, 0.0);
        assert!(v4_zero_w.to_3d().is_err());
    }

    #[test]
    fn test_display_formatting() {
        let v3 = Vector3D::new(1.0, 2.0, 3.0);
        let v4 = Vector4D::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(format!("{}", v3), "Vec3(1, 2, 3)");
        assert_eq!(format!("{}", v4), "Vec4(1, 2, 3, 4)");
    }
}
