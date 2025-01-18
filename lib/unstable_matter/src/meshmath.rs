/// MeshMath - Core Mathematical Operations Module
/// Last Updated: 2025-01-18 18:41:40 UTC
/// Author: isdood
/// Current User: isdood

use crate::arch::x86_64::instructions;
use core::f64::consts::PI;

pub struct MeshMath;

/// Core trait for mesh-aware numeric types
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
                let bits: u64 = core::mem::transmute(self);
                core::mem::transmute(bits ^ sign_bit)
            }
        })
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0.0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1.0 }
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
                    out(reg) result,
                                 in(reg) self,
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
                    out(reg) result,
                                 in(reg) self,
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
                    out(reg) result,
                                 in(reg) self,
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
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "mov rax, {}",
                    "cqo",
                    "idiv {}",
                    in(reg) self,
                                 in(reg) other,
                                 out("rax") result,
                                 out("rdx") _,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: isize;
                core::arch::asm!(
                    "neg {}",
                    out(reg) result,
                                 in(reg) self,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }
}

// Implement for usize
impl MeshValue for usize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: usize;
                core::arch::asm!(
                    "add {}, {}",
                    out(reg) result,
                                 in(reg) self,
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
                let result: usize;
                core::arch::asm!(
                    "sub {}, {}",
                    "cmovb {}, {}",  // If borrow, return 0
                    out(reg) result,
                                 in(reg) self,
                                 in(reg) other,
                                 in(reg) 0usize,
                                 options(nomem, nostack)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: usize;
                core::arch::asm!(
                    "mul {}",
                    in(reg) self,
                                 in(reg) other,
                                 out("rax") result,
                                 out("rdx") _,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        instructions::without_interrupts(|| {
            unsafe {
                let result: usize;
                core::arch::asm!(
                    "mov rax, {}",
                    "xor rdx, rdx",
                    "div {}",
                    in(reg) self,
                                 in(reg) other,
                                 out("rax") result,
                                 out("rdx") _,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        0  // usize can't be negative, return zero
    }

    #[inline(always)]
    fn mesh_zero() -> Self { 0 }

    #[inline(always)]
    fn mesh_one() -> Self { 1 }
}

impl MeshMath {
    #[inline(always)]
    pub fn sqrt(x: f64) -> f64 {
        Self::sqrt_f64(x)
    }

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
    pub fn sin(x: f64) -> f64 {
        Self::sin_f64(x)
    }

    #[inline(always)]
    pub fn sin_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            // Normalize angle to [0, 2π]
            let normalized = x % (2.0 * PI);
            unsafe {
                let mut result: f64;
                core::arch::asm!(
                    "fld qword ptr [{0}]",
                    "fsin",
                    "fstp qword ptr [{1}]",
                    in(reg) &normalized,
                                 in(reg) &mut result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn cos(x: f64) -> f64 {
        Self::cos_f64(x)
    }

    #[inline(always)]
    pub fn cos_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            // Normalize angle to [0, 2π]
            let normalized = x % (2.0 * PI);
            unsafe {
                let mut result: f64;
                core::arch::asm!(
                    "fld qword ptr [{0}]",
                    "fcos",
                    "fstp qword ptr [{1}]",
                    in(reg) &normalized,
                                 in(reg) &mut result,
                                 options(nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn tan(x: f64) -> f64 {
        Self::sin(x) / Self::cos(x)
    }

    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        Self::abs_f64(x)
    }

    #[inline(always)]
    pub fn abs_f64(x: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let mask: u64 = !(1u64 << 63);
                let bits: u64 = core::mem::transmute(x);
                core::mem::transmute(bits & mask)
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

    #[inline(always)]
    pub fn f64_to_usize(x: f64) -> usize {
        instructions::without_interrupts(|| {
            unsafe {
                let result: usize;
                core::arch::asm!(
                    "cvttsd2si {}, {}",
                    out(reg) result,
                                 in(xmm_reg) x,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn min_f64(x: f64, y: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "minsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) x,
                                 in(xmm_reg) y,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }

    #[inline(always)]
    pub fn max_f64(x: f64, y: f64) -> f64 {
        instructions::without_interrupts(|| {
            unsafe {
                let result: f64;
                core::arch::asm!(
                    "maxsd {}, {}",
                    out(xmm_reg) result,
                                 in(xmm_reg) x,
                                 in(xmm_reg) y,
                                 options(nomem, nostack, preserves_flags)
                );
                result
            }
        })
    }
}
