//! Crystal-Aware Multiplication Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 02:14:30 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathResult;

pub trait RawMul {
    fn raw_mul(&self, other: &Self) -> MathResult<Self> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3d::Vector3D;
    use crate::vector4d::Vector4D;

    #[test]
    fn test_vector3d_multiplication() -> MathResult<()> {
        let v1 = Vector3D::new(2.0, 3.0, 4.0);
        let v2 = Vector3D::new(3.0, 4.0, 5.0);
        let result = v1.mul(&v2)?;
        assert_eq!(result.x, 6.0);
        assert_eq!(result.y, 12.0);
        assert_eq!(result.z, 20.0);
        Ok(())
    }

    #[test]
    fn test_vector4d_multiplication() -> MathResult<()> {
        let v1 = Vector4D::new(2.0, 3.0, 4.0, 5.0);
        let v2 = Vector4D::new(3.0, 4.0, 5.0, 6.0);
        let result = v1.mul(&v2)?;
        assert_eq!(result.x, 6.0);
        assert_eq!(result.y, 12.0);
        assert_eq!(result.z, 20.0);
        assert_eq!(result.w, 30.0);
        Ok(())
    }
}
