//! Crystal-Aware Division Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 02:14:30 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathResult;

pub trait RawDiv {
    fn raw_div(&self, other: &Self) -> MathResult<Self> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::MathError;
    use crate::vector3d::Vector3D;
    use crate::vector4d::Vector4D;

    #[test]
    fn test_vector3d_division() -> MathResult<()> {
        let v1 = Vector3D::new(6.0, 12.0, 15.0);
        let v2 = Vector3D::new(2.0, 3.0, 5.0);
        let result = v1.div(&v2)?;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 3.0);
        Ok(())
    }

    #[test]
    fn test_vector4d_division() -> MathResult<()> {
        let v1 = Vector4D::new(6.0, 12.0, 15.0, 20.0);
        let v2 = Vector4D::new(2.0, 3.0, 5.0, 4.0);
        let result = v1.div(&v2)?;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 3.0);
        assert_eq!(result.w, 5.0);
        Ok(())
    }

    #[test]
    fn test_division_by_zero() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(0.0, 1.0, 1.0);
        assert!(matches!(v1.div(&v2), Err(MathError::DivisionByZero)));
    }
}
