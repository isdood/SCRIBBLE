// lib/unstable_matter/src/vector.rs
/// Vector Types Implementation
/// Last Updated: 2025-01-13 04:25:11 UTC
/// Author: isdood
/// Current User: isdood

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<T: PartialEq> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vector3D<f64> {
    pub fn magnitude(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

pub type FloatVector3D = Vector3D<f64>;
pub type IntVector3D = Vector3D<isize>;

impl<T: PartialEq> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_equality() {
        let v1: FloatVector3D = Vector3D::new(1.0, 2.0, 3.0);
        let v2: FloatVector3D = Vector3D::new(1.0, 2.0, 3.0);
        let v3: FloatVector3D = Vector3D::new(2.0, 2.0, 3.0);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_vector3d_copy() {
        let v1: IntVector3D = Vector3D::new(1, 2, 3);
        let v2 = v1;

        assert_eq!(v1, v2);
        assert_eq!(v1.x, 1);
        assert_eq!(v1.y, 2);
        assert_eq!(v1.z, 3);
    }

    #[test]
    fn test_vector3d_clone() {
        let v1: FloatVector3D = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = v1.clone();

        assert_eq!(v1, v2);
    }
}
