// lib/ziggy/src/lib.rs
use std::os::raw::c_double;

#[repr(C)]
pub struct Vector3D {
    x: c_double,
    y: c_double,
    z: c_double,
}

extern "C" {
    fn vector3d_dot(v1: Vector3D, v2: Vector3D) -> c_double;
    fn vector3d_magnitude(v: Vector3D) -> c_double;
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        unsafe { vector3d_dot(*self, *other) }
    }

    pub fn magnitude(&self) -> f64 {
        unsafe { vector3d_magnitude(*self) }
    }
}

// lib/ziggy/src/lib.rs (continued)
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        assert_relative_eq!(v1.dot(&v2), 32.0);
        assert_relative_eq!(v1.magnitude(), 3.7416573867739413);
    }
}
