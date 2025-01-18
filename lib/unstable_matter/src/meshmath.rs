/// MeshMath - Core Mathematical Operations Module
/// Last Updated: 2025-01-18 16:49:29 UTC
/// Author: isdood
/// Current User: isdood

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
        unsafe {
            let result: f64;
            std::arch::asm!(
                "addsd {}, {}",
                out(xmm_reg) result,
                            in(xmm_reg) self,
                            in(xmm_reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        unsafe {
            let result: f64;
            std::arch::asm!(
                "subsd {}, {}",
                out(xmm_reg) result,
                            in(xmm_reg) self,
                            in(xmm_reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        unsafe {
            let result: f64;
            std::arch::asm!(
                "mulsd {}, {}",
                out(xmm_reg) result,
                            in(xmm_reg) self,
                            in(xmm_reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        unsafe {
            let result: f64;
            std::arch::asm!(
                "divsd {}, {}",
                out(xmm_reg) result,
                            in(xmm_reg) self,
                            in(xmm_reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        unsafe {
            let sign_bit: u64 = 1u64 << 63;
            let bits: u64 = std::mem::transmute(self);
            std::mem::transmute(bits ^ sign_bit)
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

// Implement for isize
impl MeshValue for isize {
    #[inline(always)]
    fn mesh_add(self, other: Self) -> Self {
        unsafe {
            let result: isize;
            std::arch::asm!(
                "add {}, {}",
                inout(reg) self => result,
                            in(reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        unsafe {
            let result: isize;
            std::arch::asm!(
                "sub {}, {}",
                inout(reg) self => result,
                            in(reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        unsafe {
            let result: isize;
            std::arch::asm!(
                "imul {}, {}",
                inout(reg) self => result,
                            in(reg) other
            );
            result
        }
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        self / other // Use native division for isize
    }

    #[inline(always)]
    fn mesh_neg(self) -> Self {
        -self
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
        self + other
    }

    #[inline(always)]
    fn mesh_sub(self, other: Self) -> Self {
        self - other
    }

    #[inline(always)]
    fn mesh_mul(self, other: Self) -> Self {
        self * other
    }

    #[inline(always)]
    fn mesh_div(self, other: Self) -> Self {
        self / other
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
        unsafe {
            let result: f64;
            std::arch::asm!(
                "sqrtsd {}, {}",
                out(xmm_reg) result,
                            in(xmm_reg) x
            );
            result
        }
    }

    #[inline(always)]
    pub fn eq_f64(x: f64, y: f64) -> bool {
        unsafe {
            let result: u8;
            std::arch::asm!(
                "comisd {}, {}",
                "sete {}",
                in(xmm_reg) x,
                            in(xmm_reg) y,
                            out(reg_byte) result
            );
            result != 0
        }
    }

    #[inline(always)]
    pub fn isize_to_f64(x: isize) -> f64 {
        unsafe {
            let result: f64;
            std::arch::asm!(
                "cvtsi2sd {}, {}",
                out(xmm_reg) result,
                            in(reg) x
            );
            result
        }
    }
}
