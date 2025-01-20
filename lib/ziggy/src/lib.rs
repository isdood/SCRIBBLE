use thiserror::Error;
use scribe::Write;

#[derive(Debug, Error)]
pub enum ZiggyError {
    #[error("Vector operation failed")]
    VectorError,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Write for Vector3D {
    fn write(&self, f: &mut scribe::Formatter) -> scribe::Result {
        f.write_str("Vector3D(")?;
        self.x.write(f)?;
        f.write_str(", ")?;
        self.y.write(f)?;
        f.write_str(", ")?;
        self.z.write(f)?;
        f.write_str(")")
    }
}

extern "C" {
    fn vector3d_dot(v1: Vector3D, v2: Vector3D) -> f64;
    fn vector3d_magnitude(v: Vector3D) -> f64;
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
