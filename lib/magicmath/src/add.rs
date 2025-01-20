//! Crystal-Aware Addition Operations
//! ==============================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 02:12:57 UTC
//! Version: 0.1.0
//! License: MIT

use errors::MathResult;

pub trait RawAdd {
    fn raw_add(&self, other: &Self) -> MathResult<Self> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3d::Vector3D;
    use crate::vector4d::Vector4D;

    #[test]
    fn test_vector3d_addition() -> MathResult<()> {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2)?;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
        Ok(())
    }

    #[test]
    fn test_vector4d_addition() -> MathResult<()> {
        let v1 = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vector4D::new(5.0, 6.0, 7.0, 8.0);
        let result = v1.add(&v2)?;
        assert_eq!(result.x, 6.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, 10.0);
        assert_eq!(result.w, 12.0);
        Ok(())
    }
}
