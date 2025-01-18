/// MeshMath - Core Mathematical Operations Module
/// Last Updated: 2025-01-18 17:52:20 UTC
/// Author: isdood
/// Current User: isdood

use crate::arch::x86_64::instructions;

pub struct MeshMath;

// Generic trait definitions for mesh operations
pub trait MeshValue: Copy {
    fn mesh_add(self, other: Self) -> Self;
    fn mesh_sub(self, other: Self) -> Self;
    fn mesh_mul(self, other: Self) -> Self;
    fn mesh_div(self, other: Self) -> Self;
    fn mesh_neg(self) -> Self;
    fn mesh_zero() -> Self;
    fn mesh_one() -> Self;
}

// Implement for f64
impl MeshValue for f64 {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "addsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) self,
                                 in(xmm_reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "subsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) self,
                                 in(xmm_reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "mulsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) self,
                                 in(xmm_reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "divsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) self,
                                 in(xmm_reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let sign_bit: u64 = 1u64 << 63;
                let bits: u64 = std::mem::transmute(self);
                std::mem::transmute(bits ^ sign_bit)
            }
        })
    }

    #[inline(always)]
    fn mesh_zero() -> Self {
        0.0
    }

    #[inline(always)]
    fn mesh_one() -> Self {
        1.0
    }
}

// Implement for isize
impl MeshValue for isize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "add {}, {}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "sub {}, {}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "imul {}, {}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            self / other // Use native division for isize
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        instructions::without_interrupts(|| {
            -self
        })
    }

    #[inline(always)]
    fn mesh_zero() -> Self {
        0
    }

    #[inline(always)]
    fn mesh_one() -> Self {
        1
    }
}

impl MeshValue for usize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            self + other
        })
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            self - other
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            self * other
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            self / other
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        panic!("Cannot negate unsigned value")
    }

    #[inline(always)]
    fn mesh_zero() -> Self {
        0
    }

    #[inline(always)]
    fn mesh_one() -> Self {
        1
    }
}

impl MeshMath {
    #[inline(always)]
    pub fn sqrt_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "sqrtsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) x,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn sin_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let mut result: f64;
                core::arch::asm!(
                    "fld qword ptr [{0}]",
                    "fsin",
                    "fstp qword ptr [{1}]",
                    in(reg) &x,
                                 in(reg) &mut result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn cos_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let mut result: f64;
                core::arch::asm!(
                    "fld qword ptr [{0}]",
                    "fcos",
                    "fstp qword ptr [{1}]",
                    in(reg) &x,
                                 in(reg) &mut result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn eq_f64(x: f64, y: f64) -> bool {
        instructions::without_interrupts(|| {
            unsafe {
                let result: u8;
                core::arch::asm!(
                    "comisd {}, {}",
                    "sete {}",
                    in(xmm_reg) x,
                                 in(xmm_reg) y,
                                 out(reg_byte) result,
                                 options(nomem, nostack, preserves_flags)
                );
                result != 0
            }
        })
    }

    #[inline(always)]
    pub fn isize_to_f64(x: isize) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "cvtsi2sd {}, {}",
                    out(xmm_reg) result,
                                 in(reg) x,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn abs_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let mask: u64 = !(1u64 << 63);
                let bits: u64 = std::mem::transmute(x);
                std::mem::transmute(bits & mask)
            }
        })
    }

    #[inline(always)]
    pub fn usize_to_f64(x: usize) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "cvtsi2sd {}, {}",
                    out(xmm_reg) result,
                                 in(reg) x,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_ops() {
        assert_eq!(1.0.mesh_add(2.0), 3.0);
        assert_eq!(3.0.mesh_sub(2.0), 1.0);
        assert_eq!(2.0.mesh_mul(3.0), 6.0);
        assert_eq!(6.0.mesh_div(2.0), 3.0);
        assert_eq!(1.0.mesh_neg(), -1.0);
    }

    #[test]
    fn test_math_funcs() {
        assert!(MeshMath::eq_f64(MeshMath::sqrt_f64(4.0), 2.0));
        assert!(MeshMath::eq_f64(MeshMath::sin_f64(0.0), 0.0));
        assert!(MeshMath::eq_f64(MeshMath::cos_f64(0.0), 1.0));
        assert_eq!(MeshMath::abs_f64(-1.0), 1.0);
    }

    #[test]
    fn test_conversions() {
        assert_eq!(MeshMath::isize_to_f64(42), 42.0);
        assert_eq!(MeshMath::usize_to_f64(42), 42.0);
    }

    #[test]
    fn test_unsigned_ops() {
        let a: usize = 42;
        let b: usize = 12;
        assert_eq!(a.mesh_add(b), 54);
        assert_eq!(a.mesh_sub(b), 30);
        assert_eq!(a.mesh_mul(b), 504);
        assert_eq!(a.mesh_div(b), 3);
    }
}
