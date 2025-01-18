/// MeshMath - Core Mathematical Operations Module
/// Last Updated: 2025-01-18 19:05:44 UTC
/// Author: isdood
/// Current User: isdood

use crate::arch::x86_64::instructions::interrupts::without_interrupts;

#[derive(Debug)]
pub struct MeshMath;

/// Core trait for mesh-aware numeric types
pub trait MeshValue: Copy {
    fn mesh_add(self, other: Self) -> Self;
    fn mesh_sub(self, other: Self) -> Self;
    fn mesh_mul(self, other: Self) -> Self;
    fn mesh_div(self, other: Self) -> Self;
    fn mesh_neg(self) -> Self;
    fn mesh_magnitude(self) -> f64;
    fn mesh_normalize(self) -> Self;
    fn mesh_zero() -> Self;
    fn mesh_one() -> Self;
}

// Implementation for isize
impl MeshValue for isize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "add {0}, {1}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "sub {0}, {1}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "imul {0}, {1}",
                    inout(reg) self => result,
                                 in(reg) other,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        without_interrupts(|| {
            if other == 0 {
                return 0;
            }
            unsafe {
                let mut result: isize;
                core::arch::asm!(
                    "mov rax, {1}",
                    "cqo",
                    "idiv {2}",
                    "mov {0}, rax",
                    out(reg) result,
                                 in(reg) self,
                                 in(reg) other,
                                 lateout("rax") _,
                                 lateout("rdx") _,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "neg {0}",
                    inout(reg) self => result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 {
        (self.abs() as f64)
    }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0 {
            0
        } else if self > 0 {
            1
        } else {
            -1
        }
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

// Implementation for f64
impl MeshValue for f64 {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: f64 = 0.0;
                core::arch::asm!(
                    "movsd xmm0, [{src1}]",
                    "addsd xmm0, [{src2}]",
                    "movsd [{dest}], xmm0",
                    src1 = in(reg) &self,
                                 src2 = in(reg) &other,
                                 dest = in(reg) &result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: f64 = 0.0;
                core::arch::asm!(
                    "movsd xmm0, [{src1}]",
                    "subsd xmm0, [{src2}]",
                    "movsd [{dest}], xmm0",
                    src1 = in(reg) &self,
                                 src2 = in(reg) &other,
                                 dest = in(reg) &result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: f64 = 0.0;
                core::arch::asm!(
                    "movsd xmm0, [{src1}]",
                    "mulsd xmm0, [{src2}]",
                    "movsd [{dest}], xmm0",
                    src1 = in(reg) &self,
                                 src2 = in(reg) &other,
                                 dest = in(reg) &result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: f64 = 0.0;
                core::arch::asm!(
                    "movsd xmm0, [{src1}]",
                    "divsd xmm0, [{src2}]",
                    "movsd [{dest}], xmm0",
                    src1 = in(reg) &self,
                                 src2 = in(reg) &other,
                                 dest = in(reg) &result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        without_interrupts(|| {
            unsafe {
                let result: f64 = 0.0;
                core::arch::asm!(
                    "movsd xmm0, [{src}]",
                    "xorpd xmm1, xmm1",
                    "subsd xmm1, xmm0",
                    "movsd [{dest}], xmm1",
                    src = in(reg) &self,
                                 dest = in(reg) &result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_magnitude(self) -> f64 {
        self.abs()
    }

    #[inline(always)]
    fn mesh_normalize(self) -> Self {
        if self == 0.0 {
            0.0
        } else {
            self / self.abs()
        }
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isize_ops() {
        assert_eq!(5_isize.mesh_add(3), 8);
        assert_eq!(5_isize.mesh_sub(3), 2);
        assert_eq!(5_isize.mesh_mul(3), 15);
        assert_eq!(15_isize.mesh_div(3), 5);
        assert_eq!(5_isize.mesh_neg(), -5);
    }

    #[test]
    fn test_f64_ops() {
        assert_eq!(5.0_f64.mesh_add(3.0), 8.0);
        assert_eq!(5.0_f64.mesh_sub(3.0), 2.0);
        assert_eq!(5.0_f64.mesh_mul(3.0), 15.0);
        assert_eq!(15.0_f64.mesh_div(3.0), 5.0);
        assert_eq!(5.0_f64.mesh_neg(), -5.0);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(5_isize.mesh_normalize(), 1);
        assert_eq!((-5_isize).mesh_normalize(), -1);
        assert_eq!(0_isize.mesh_normalize(), 0);

        assert_eq!(5.0_f64.mesh_normalize(), 1.0);
        assert_eq!((-5.0_f64).mesh_normalize(), -1.0);
        assert_eq!(0.0_f64.mesh_normalize(), 0.0);
    }
}
