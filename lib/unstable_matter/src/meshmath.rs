//! MeshMath - Core Mathematical Operations Module
//! Last Updated: 2025-01-18 19:20:54 UTC
//! Author: isdood

/// Core mathematical operations for mesh-based computations
#[derive(Debug, Clone, Copy)]
pub struct MeshMath;

impl MeshMath {
    /// Convert isize to f64
    #[inline(always)]
    pub fn isize_to_f64(value: isize) -> f64 {
        value as f64
    }

    /// Compare two f64 values for equality with epsilon
    #[inline(always)]
    pub fn eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < std::f64::EPSILON
    }

    /// Calculate square root of f64
    #[inline(always)]
    pub fn sqrt_f64(value: f64) -> f64 {
        value.sqrt()
    }
}

/// Trait for mesh-compatible numeric types
pub trait MeshValue: Copy + Clone + std::fmt::Debug {
    fn mesh_add(self, other: Self) -> Self;
    fn mesh_sub(self, other: Self) -> Self;
    fn mesh_mul(self, other: Self) -> Self;
    fn mesh_div(self, other: Self) -> Self;
    fn mesh_neg(self) -> Self;
    fn mesh_magnitude(self) -> f64;
    fn mesh_normalize(self) -> Self;
    fn mesh_zero() -> Self;
    fn mesh_one() -> Self;
    fn as_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;
}

// Implementation for f64
impl MeshValue for f64 {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self { self - other }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if MeshMath::eq_f64(other, 0.0) { 0.0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { -self }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { self.abs() }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if MeshMath::eq_f64(self, 0.0) { 0.0 } else { self / self.abs() }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0.0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1.0 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value }
}

// Implementation for isize
impl MeshValue for isize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self { self - other }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if other == 0 { 0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { -self }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { self.abs() as f64 }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0 { 0 } else { self / self.abs() }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self as f64 }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value as isize }
}

// Implementation for usize
impl MeshValue for usize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self { self + other }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        if other > self { 0 } else { self - other }
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self { self * other }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        if other == 0 { 0 } else { self / other }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self { 0 }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 { self as f64 }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0 { 0 } else { 1 }
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }

    #[inline(always)]
    fn as_f64(self) -> f64 { self as f64 }

    #[inline(always)]
    fn from_f64(value: f64) -> Self { value as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_math_conversions() {
        assert_eq!(MeshMath::isize_to_f64(5), 5.0);
        assert!(MeshMath::eq_f64(5.0, 5.0));
        assert!(MeshMath::sqrt_f64(25.0) - 5.0 < std::f64::EPSILON);
    }

    #[test]
    fn test_mesh_value_implementations() {
        // Test f64
        assert_eq!(5.0_f64.mesh_add(3.0), 8.0);
        assert_eq!(5.0_f64.mesh_sub(3.0), 2.0);
        assert_eq!(5.0_f64.mesh_mul(3.0), 15.0);
        assert_eq!(15.0_f64.mesh_div(3.0), 5.0);

        // Test isize
        assert_eq!(5_isize.mesh_add(3), 8);
        assert_eq!(5_isize.mesh_sub(3), 2);
        assert_eq!(5_isize.mesh_mul(3), 15);
        assert_eq!(15_isize.mesh_div(3), 5);

        // Test usize
        assert_eq!(5_usize.mesh_add(3), 8);
        assert_eq!(5_usize.mesh_sub(3), 2);
        assert_eq!(5_usize.mesh_mul(3), 15);
        assert_eq!(15_usize.mesh_div(3), 5);
    }
}
